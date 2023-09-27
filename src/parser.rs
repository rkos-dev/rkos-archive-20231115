use lazy_static::lazy_static;
use petgraph::graph::NodeIndex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::execute::ScriptTp;
use crate::execute::RustTp;
use crate::execute::DownloadTp;
use crate::execute::RustFn;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageTemplete {
    pub package_name: String,
    pub package_version: String,
    pub package_url: String,
    pub depends: Vec<String>,
    pub rundeps: Vec<String>,
    pub testdeps: Vec<String>,
    pub before: Vec<String>,
    pub optdeps: Vec<String>,
    pub script_path: String,
    pub running_type: String,
    pub envs: Vec<Env>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VirtualDeps{
    pub v_depends:Vec<String>,
}

impl JsonObj<VirtualDeps> for VirtualDeps{}

//const package templete support to create templete
lazy_static! {
    static ref PACKAGE_TEMPLETE: PackageTemplete = PackageTemplete {
        package_name: "".to_string(),
        package_version: "".to_string(),
        package_url: "".to_string(),
        script_path: "".to_string(),
        running_type: "".to_string(),
        envs: Vec::new(),
        depends: Vec::new(),
        rundeps: Vec::new(),
        testdeps: Vec::new(),
        before: Vec::new(),
        optdeps: Vec::new(),
    };
}

//package path could used for package templete parser or store,if provided package dir path
pub struct PackagePaths {
    script: PathBuf,
    src: PathBuf,
    json: PathBuf,
}

impl PackagePaths {
    // create new paths from path and package name
    fn new(mut path: PathBuf, package_name: String) -> Option<PackagePaths> {
        if !path.exists() {
            return None;
        }
        path = path.join(package_name);

        Some(PackagePaths {
            script: path.clone().join("script"),
            src: path.clone().join("src"),
            json: path.clone().join("config.json"),
        })
    }

    // get path from package name,path from config file
    fn get_paths(packages_path: PathBuf, package_name: String) -> Option<PackagePaths> {
        match fs::read_dir(packages_path.clone()) {
            Ok(entrys) => {
                let package_path = PackagePaths::new(packages_path.clone(), package_name)?;

                let path_vec = vec![
                    package_path.script.clone(),
                    package_path.src.clone(),
                    package_path.json.clone(),
                ];

                for entry in entrys {
                    let entry = match entry {
                        Ok(v) => v,
                        Err(_) => return None,
                    };
                    if !path_vec.contains(&entry.path().to_path_buf()) {
                        return None;
                    }
                }
                Some(package_path)
            }
            Err(_) => None,
        }
    }

    // create new dirs and store json file
    fn store_in_disk(&self, package_templete: PackageTemplete) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(self.script.clone())?;
        fs::create_dir_all(self.src.clone())?;

        //TODO: need add result deal
        package_templete.store(self.json.clone());
        Ok(())
    }
}

pub struct PackageGraphRawData {
    //store package index in package vec
    pub package_index: usize,
    //store package name and package idx with hashmap
    pub package_name_to_index: HashMap<String, usize>,
    //store pacakge templete which parser from json file
    pub package_vec: Vec<PackageTemplete>,
    // store node index,which used for create graph
    pub package_node_vec: Vec<NodeIndex>,

    pub virtual_pacakges:VirtualDeps,
}

impl PackageGraphRawData {
    //test
    //    let package_raw = PackageGraphRawData::new(["package"].into_iter().collect()).unwrap();
    //    for (key, value) in package_raw.package_name_to_index {
    //        println!("{} {}", key, value);
    //        println!("{:?}", package_raw.package_vec[value]);
    //    }
    pub fn new(packages_path: PathBuf) -> Option<PackageGraphRawData> {
        let mut package_raw = PackageGraphRawData {
            package_index: 0,
            package_name_to_index: HashMap::new(),
            //parse package from path
            package_vec: PackageTemplete::parse_all(packages_path.clone())?,

            package_node_vec: Vec::new(),

            virtual_pacakges:VirtualDeps::load(packages_path.join("virtual")),
        };

        //filing hashmap with package name and package index
        for i in &package_raw.package_vec {
            package_raw
                .package_name_to_index
                .insert(i.package_name.clone(), package_raw.package_index);
            package_raw.package_index = package_raw.package_index + 1;
        }

        //package_raw with only node_vec empty
        Some(package_raw)
    }

    //push node in package_node_vec
    pub fn filling_node(&mut self, node: NodeIndex) {
        self.package_node_vec.push(node);
    }

    pub fn index2name(&self,index:usize)->String{
        self.package_vec[index].package_name.clone()
    }

    // get node index copy
    pub fn node(&self, package_name: String) -> Option<NodeIndex> {
        let index = self.package_name_to_index.get(&package_name)?.clone();
        self.package_node_vec.get(index).copied()
    }
}

pub trait JsonObj<T>
where
    T: DeserializeOwned + Serialize,
    Self: Serialize,
{
    //load json
    fn load(file_path: PathBuf) -> T {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let value: T = serde_json::from_reader(reader).unwrap();
        value
    }

    //store json
    fn store(&self, file_path: PathBuf) {
        let json_string_stream = serde_json::to_string(self).unwrap();
        File::create(file_path)
            .unwrap()
            .write_all(json_string_stream.as_bytes())
            .expect("store to json failed");
    }
}

impl JsonObj<PackageTemplete> for PackageTemplete {}

impl PackageTemplete {
    //test
    //    let package = PackageTemplete::parse_all(["package"].into_iter().collect());
    //    match package {
    //        Ok(v) => {
    //            for i in v {
    //                println!("{:?}", i);
    //            }
    //        }
    //        Err(e) => {
    //            println!("{:?}", e);
    //        }
    //    }
    pub fn parse_all(packages_path: PathBuf) -> Option<Vec<PackageTemplete>> {
        let mut package_vec: Vec<PackageTemplete> = Vec::new();

        //walkdir include son dir
        for entry in WalkDir::new(packages_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().extension() == Some(OsStr::new("json")) {
                package_vec.push(PackageTemplete::load(entry.path().to_path_buf()));
            }
        }
        Some(package_vec)
    }

    //new templete
    pub fn new() -> PackageTemplete {
        PACKAGE_TEMPLETE.clone()
    }
    
    pub fn to_script_tp(&self,work_path:PathBuf,log_path:PathBuf)->Option<ScriptTp>{
        Some(ScriptTp{
            work_path:work_path,
            script_path:self.script_path.clone().into(),
            package_name:self.package_name.clone(),
            log_path:log_path,
            envs:self.envs.clone(),
        })
    }
    
    pub fn to_rust_tp(&self,func:RustFn)->Option<RustTp>{
        Some(RustTp{
            rfn:func,
        })
    }

    pub fn to_download_tp(&self,target_path:PathBuf)->Option<DownloadTp>{
        Some(DownloadTp { target_path: target_path, url: self.package_url.clone(), name: self.package_name.clone() })
    }
}

//target store path should not being created,and package templete could be any package ,and then this func will store it in disk
pub fn create_new_package(
    target_store_path: PathBuf,
    package_templete: PackageTemplete,
) -> Result<(), Box<dyn Error>> {
    let package_path =
        match PackagePaths::new(target_store_path, package_templete.package_name.clone()) {
            Some(v) => v,
            None => {
                return Err("Create new package failed".into());
            }
        };
    package_path.store_in_disk(package_templete.clone())?;
    Ok(())
}
