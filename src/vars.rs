use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::parser::JsonObj;

#[derive(Parser)]
#[command(name = "rbuilder")]
#[command(author = "xyyy <xyyy1420@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about="Mange rkos package and build rkos",long_about=None)]
pub struct Cli {
    #[arg(long)]
    node: String,
    #[arg(long)]
    graph: String,
}

lazy_static! {
    pub static ref BASE_CONFIG: BaseConfig = BaseConfig::load(["hello", "json"].iter().collect());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostInfo {
    pub target_part: String,
    pub stop_flag: String,
}

// scripte path in base config
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptsPath {
    pub root: String,
    pub build_base_packages: String,
    pub build_temp_toolchains: String,
    pub build_rust_support_packages: String,
    pub chroot: String,
    pub clean: String,
    pub prepare: String,
    pub release: String,
    pub sysconfig: String,
}

// base config
#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub root: String,
    pub rust_support_packages: String,
    pub package_config: String,
}

// path info in base config
#[derive(Debug, Serialize, Deserialize)]
pub struct PathInfo {
    pub root: String,
    pub package_source: String,
    pub package_build: String,
    pub package_patches: String,
    pub install_path: String,
}

// env config
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvsInfo {
    pub name: String,
    pub value: String,
}

// env config vec
#[derive(Debug, Serialize, Deserialize)]
pub struct Envs {
    pub envs: Vec<EnvsInfo>,
}

// base config
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub host_info: HostInfo,
    pub scripts_path: ScriptsPath,
    pub configs: Configs,
    pub path: PathInfo,
    pub envs: Vec<EnvsInfo>,
}

impl JsonObj<BaseConfig> for BaseConfig {}
