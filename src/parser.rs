use petgraph::graph::NodeIndex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageTemplete {
    pub package_name: String,
    pub file_name: String,
    pub url: String,
    pub last_version: String,
    pub current_version: String,
    pub running_type: String,
    pub script_path: String,
    pub depdencies: Vec<String>,
    pub envs: Vec<Env>,
}

pub struct PackageGraphRawData {
    pub package_index: u64,
    pub package_name_to_index: HashMap<String, u64>,
    pub package_vec: Vec<PackageTemplete>,
    pub package_node_vec: Vec<NodeIndex>,
}

pub trait JsonObj<T>
where
    T: DeserializeOwned + Serialize,
{
    fn load(file_path: PathBuf) -> T {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let value: T = serde_json::from_reader(reader).unwrap();
        value
    }
    fn store(json_obj: &T, file_path: PathBuf) {
        let json_string_stream = serde_json::to_string(json_obj).unwrap();
        File::create(file_path)
            .unwrap()
            .write_all(json_string_stream.as_bytes())
            .expect("store to json failed");
    }
}

impl PackageTemplete {
    pub fn parse_all(dir_path: PathBuf) -> Vec<PackageTemplete> {
        Vec::new()
    }

    pub fn new()

    //    pub fn new_templete(target_path: PathBuf, package_name: String) {
    //        let mut target_path = target_path.to_owned().push(package_name);
    //        fs::create_dir_all(target_path.to_owned()).unwrap();
    //
    //        fs::create_dir(target_path.push("data")).unwrap();
    //
    //        target_path = target_path.pop();
    //
    //        let json_data = PackageTemplete {
    //            package_name: "".to_string(),
    //            file_name: "".to_string(),
    //            url: "".to_string(),
    //            last_version: "".to_string(),
    //            current_version: "".to_string(),
    //            running_type: "".to_string(),
    //            script_path: "".to_string(),
    //            depdencies: vec!["".to_string()],
    //            envs: vec![Env {
    //                name: "".to_string(),
    //                value: "".to_string(),
    //            }],
    //        };
    //        json_data.store(target_path.push(package_name.push(".json")));
    //
    //        let script_data = "#!/bin/bash".to_string();
    //        let script_file =
    //            File::create_new(target_path.pop().push(package_name.push(".sh"))).unwrap();
    //        script_file.write_all(script_data);
    //    }
}
