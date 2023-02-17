use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

lazy_static! {
    pub static ref ROOT_DIR: PathBuf = env::current_dir().unwrap();
    pub static ref BASE_CONFIG: BaseConfig = {
        let temp = parse_json("configs/base_config.json");
        match temp {
            Ok(v) => v,
            Err(_e) => panic!("Cannot load base config"),
        }
    };
    pub static ref BASE_PACKAGES: BasePackages = {
        let temp = parse_json("configs/base_packages.json");
        match temp {
            Ok(v) => v,
            Err(_e) => panic!("Cannot load base packages"),
        }
    };
    pub static ref CROSS_COMPILE_PACKAGES: CrossCompilePackages = {
        let temp = parse_json("configs/cross_compile_packages.json");
        match temp {
            Ok(v) => v,
            Err(_e) => panic!("Cannot load cross compile packages"),
        }
    };
    pub static ref HOST_PACKAGES: HostPackage = {
        let temp = parse_json("configs/host_packages.json");
        match temp {
            Ok(v) => v,
            Err(_e) => panic!("Cannot load host packages"),
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostPackage {
    pub host_packages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub url: String,
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchInfo {
    pub name: String,
    pub url: String,
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePackages {
    pub base_packages: Vec<PackageInfo>,
    pub package_patches: Vec<PatchInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub lfs_partition: String,
    pub lfs_env: String,
    pub base_packages: String,
    pub cross_compile_packages: String,
    pub package_sources_path: String,
    pub host_install_cmd: String,
    pub cross_compile_script_path: String,
    pub base_compile_script_path: String,
    pub decompress_script: String,
    pub enter_chroot_script_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossCompilePackages {
    pub cross_compile_toolchains: Vec<String>,
    pub cross_compile_packages: Vec<String>,
}

pub fn parse_json<T: serde::de::DeserializeOwned>(
    json_file_path: &str,
) -> Result<T, Box<dyn Error>> {
    let file = File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let value: T = serde_json::from_reader(reader)?;
    Ok(value)
}
