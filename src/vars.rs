use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use log::{error, info};
use requestty::{Answer, Answers, Question};

#[derive(Parser)]
#[command(name = "rkos_builder")]
#[command(author = "xyyy <xyyy1420@gmail>")]
#[command(version = "0.0.1")]
pub struct Cli {
    #[arg(value_enum)]
    pub build_option: BuildOption,

    #[arg(short, long, value_name = "DIR")]
    pub config: Option<PathBuf>,

    #[arg(value_enum)]
    pub operate: StartMode,

    //编译中断后，可以填写该字段，避免重复编译成功的部分
    #[arg(default_value_t = String::from("NULL"), value_name = "PACKAGE_NAME")]
    pub package_name: String,

    #[arg(short,long,action=clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StartMode {
    Start,
    Reset,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BuildOption {
    Build,
    HostConfig,
    PackageDownload,
    BuildTempToolchains,
    BuildBasePackages,
    BuildRustSupportPackageAndKernel,
    ConfigTargetSystem,
    InstallGrub,
    CleanUp,
}

lazy_static! {
    pub static ref DISK_INFO:Answers=req_user_input();
    pub static ref BOOT_PARTUUID:String=get_uuid(DISK_INFO["target_boot_part"].clone(), false);
    pub static ref BOOT_UUID:String=get_uuid(DISK_INFO["target_boot_part"].clone(), true);
    pub static ref ROOT_PARTUUID:String=get_uuid(DISK_INFO["target_root_part"].clone(), false);
    pub static ref ROOT_UUID:String=get_uuid(DISK_INFO["target_root_part"].clone(), true);
    pub static ref ROOT_DIR: PathBuf = env::current_dir().unwrap();
    pub static ref BASE_CONFIG: BaseConfig = {
        let temp = parse_json(["configs", "base_configs.json"].iter().collect());
        match temp {
            Ok(v) => v,
            Err(e) => panic!("Cannot load base config , Err msg: {}",e),
        }
    };
    pub static ref STOP_FLAG: PathBuf = PathBuf::from(&BASE_CONFIG.host_info.stop_flag);
    pub static ref ALL_PACKAGES: AllPackages = {
//        let temp = parse_json("configs/all_packages.json");
        let temp = parse_json(
            [&BASE_CONFIG.configs.root, &BASE_CONFIG.configs.package_info]
                .iter()
                .collect(),
        );
        match temp {
            Ok(v) => v,
            Err(e) => panic!("Cannot load all packages , Err msg: {}",e),
        }
    };
    pub static ref RUST_SUPPORT_PACKAGES: RustSupportPackages = {
//        let temp = parse_json("configs/cross_compile_packages.json");
        let temp = parse_json([&BASE_CONFIG.configs.root,&BASE_CONFIG.configs.rust_support_packages].iter().collect());
        match temp {
            Ok(v) => v,
            Err(e) => panic!("Cannot load cross compile packages, Err msg: {}",e),
        }
    };

    pub static ref CROSS_COMPILE_PACKAGES: CrossCompilePackages = {
//        let temp = parse_json("configs/cross_compile_packages.json");
        let temp = parse_json([&BASE_CONFIG.configs.root,&BASE_CONFIG.configs.temp_toolchains].iter().collect());
        match temp {
            Ok(v) => v,
            Err(e) => panic!("Cannot load cross compile packages, Err msg: {}",e),
        }
    };
    pub static ref BASE_PACKAGES: BasePackages = {
//        let temp = parse_json("configs/base_packages.json");
        let temp = parse_json([&BASE_CONFIG.configs.root,&BASE_CONFIG.configs.base_packages].iter().collect());
        match temp {
            Ok(v) => v,
            Err(e) => panic!("Cannot load base packages , Err msg: {}",e),
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostInfo {
    pub target_part: String,
    pub stop_flag: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub root: String,
    pub package_info: String,
    pub base_packages: String,
    pub temp_toolchains: String,
    pub rust_support_packages: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathInfo {
    pub root: String,
    pub package_source: String,
    pub package_build: String,
    pub package_patches: String,
    pub install_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvsInfo {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Envs {
    pub envs: Vec<EnvsInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub host_info: HostInfo,
    pub scripts_path: ScriptsPath,
    pub configs: Configs,
    pub path: PathInfo,
    pub envs: Vec<EnvsInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePackagesInfo {
    pub name: String,
    pub package_name: String,
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePackages {
    pub base_packages: Vec<BasePackagesInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RustSupportPackageInfo {
    pub name: String,
    pub package_name: String,
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RustSupportPackages {
    pub rust_support_packages: Vec<RustSupportPackageInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPackages {
    pub all_packages: Vec<PackageInfo>,
    pub package_patches: Vec<PatchInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossCompilePackagesInfo {
    pub name: String,
    pub package_name: String,
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossCompilePackages {
    pub cross_compile_toolchains: Vec<CrossCompilePackagesInfo>,
    pub cross_compile_packages: Vec<CrossCompilePackagesInfo>,
    pub after_chroot_packages: Vec<CrossCompilePackagesInfo>,
}

pub fn parse_json<T: serde::de::DeserializeOwned>(
    json_file_path: PathBuf,
) -> Result<T, Box<dyn Error>> {
    info!("{:?}", json_file_path);
    let file = File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let value: T = serde_json::from_reader(reader)?;
    Ok(value)
}

pub fn req_user_input() -> Answers {
    let option = get_blkid_output();

    let questions = vec![
        Question::select("target_boot_part")
            .message("Which partition you want to use as a boot partition?")
            .choices(option.clone())
            .build(),
        Question::select("target_root_part")
            .message("Which partition you want to use as a root partition?")
            .choices(option.clone())
            .build(),
    ];

    match requestty::prompt(questions) {
        Ok(v) => return v,
        Err(e) => {
            error!("Failed get user input");
            panic!();
        }
    }
}

pub fn get_blkid_output() -> Vec<String> {
    let blkid = String::from_utf8(Command::new("blkid").output().unwrap().stdout);
    let mut lines: Vec<String> = Vec::new();
    match blkid {
        Ok(v) => {
            let cursor = Cursor::new(v.as_bytes());
            for line in cursor.lines().into_iter() {
                if let Ok(v) = line {
                    lines.push(v);
                }
            }
        }
        Err(e) => error!("Cannot get blkid output Err msg: {}", e),
    }
    lines
}

pub fn get_uuid(value: Answer, uuid: bool) -> String {
    match uuid {
        true => {
            if let Answer::ListItem(s) = value {
                let pattern = Regex::new("UUID=\"(.*?)\"").unwrap();
                for cap in pattern.captures_iter(&s.text) {
                    return cap[1].to_string();
                }
                return "NULL".to_string();
            } else {
                return "NULL".to_string();
            }
        }
        false => {
            if let Answer::ListItem(s) = value {
                let pattern = Regex::new("PARTUUID=\"(.*?)\"").unwrap();
                for cap in pattern.captures_iter(&s.text) {
                    return cap[1].to_string();
                }
                return "NULL".to_string();
            } else {
                return "NULL".to_string();
            }
        }
    }
}
