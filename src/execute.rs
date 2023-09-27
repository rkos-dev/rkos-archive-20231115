use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use download_rs::sync_download::Download;

use crate::parser::Env;
use crate::parser::PackageTemplete;

#[derive(Clone)]
pub struct DownloadTp {
    pub target_path: PathBuf,
    pub url: String,
    pub name: String,
}

#[derive(Clone)]
pub struct ScriptTp {
    pub work_path: PathBuf,
    pub script_path: PathBuf,
    pub package_name: String,
    pub log_path: PathBuf,
    pub envs: Vec<Env>,
}

pub type RustFn = fn() -> Result<(), Box<dyn Error>>;
#[derive(Clone)]
pub struct RustTp {
    pub rfn: RustFn,
}

#[derive(Clone)]
pub struct ExecuteOption {
    pub running_type: String,
    pub download: Option<DownloadTp>,
    pub script: Option<ScriptTp>,
    pub rust: Option<RustTp>,
}

#[derive(Clone)]
pub struct Mission {
    missions: Vec<ExecuteOption>,
    mission_count: u64,
}

impl Mission {
    pub fn new() -> Mission {
        Mission {
            missions: Vec::new(),
            mission_count: 0,
        }
    }

    pub fn push(&mut self,missioin_node:ExecuteOption){
        self.missions.push(missioin_node);
        self.mission_count=self.mission_count+1;
    }

    pub fn execute(self){
        let mut execute_count:u64=0;
        for mission in self.missions{
            match mission.execute(){
                Ok(_)=>{
                    execute_count=execute_count+1;
                }
                Err(e)=>{
                    println!("execute failed {}",e.to_string());
                    break;
                }
            }

        }
        println!("{} missions success execute, {} mission failed",&execute_count,self.mission_count-execute_count);
    }
}

fn script_execute(script: ScriptTp) -> Result<(), Box<dyn Error>> {
    let mut filter_env: HashMap<String, String> = HashMap::new();
    for i in &script.envs {
        filter_env.insert(i.name.clone(), i.value.clone());
    }

    let log_stdout_path = script
        .log_path
        .clone()
        .join(format!("{}-log.txt", script.package_name.clone()));
    let log_stderr_path = script
        .log_path
        .clone()
        .join(format!("{}-err.txt", script.package_name.clone()));

    let stdout_file = File::open(log_stdout_path).unwrap();
    let stderr_file = File::open(log_stderr_path).unwrap();
    let log_stdio = Stdio::from(stdout_file);
    let log_stderr = Stdio::from(stderr_file);

    let status = Command::new("bash")
        .arg("-e")
        .arg(script.script_path.clone())
        .env_clear()
        .envs(&filter_env)
        .current_dir(script.work_path.clone())
        .stdout(log_stdio)
        .stderr(log_stderr)
        .status()
        .expect(&format!(
            "failed to execute process {}",
            script.package_name.clone()
        ));

    match status.success() {
        true => return Ok(()),
        false => return Err("failed".into()),
    };
}

fn download_execute(download: DownloadTp) -> Result<(), Box<dyn Error>> {
    //    let target_path = vec![download.target_path.as_str(), download.name.as_str()];
    let target_path = download.target_path.join(download.name.as_str());
    //    let target_path: PathBuf = target_path.iter().collect();
    let target_path: String = target_path.into_os_string().into_string().unwrap();
    let download_task = Download::new(&download.url, Some(&target_path), None);

    match download_task.download() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}

impl ExecuteOption {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self.running_type.as_str() {
            "sh" => match &self.script {
                Some(v) => {
                    return script_execute(v.clone());
                }
                None => return Err("none script".into()),
            },

            ///    let exec=ExecuteOption{
            ///        running_type:"download".to_string(),
            ///        script:None,
            ///        rust:None,
            ///        download:Some(DownloadTp{
            ///            target_path:"./".to_string(),
            ///            url:"https://download.savannah.gnu.org/releases/acl/acl-2.3.1.tar.xz".to_string(),
            ///            name:"acl".to_string(),
            ///        })
            ///    };
            ///    exec.execute();
            "download" => match &self.download {
                Some(v) => {
                    return download_execute(v.clone());
                }
                None => return Err("none download tp".into()),
            },
            "rust" => {
                return Err("find unknown execute type".into());
            }
            _ => {
                return Err("find unknown execute type".into());
            }
        };
    }
}
