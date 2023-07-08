use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Env {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PachageTemplete {
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

pub fn parser_json<T: serde::de::DeserializeOwned>(
    file_path: PathBuf,
) -> Result<T, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let value: T = serde_json::from_reader(reader)?;
    Ok(value)
}
