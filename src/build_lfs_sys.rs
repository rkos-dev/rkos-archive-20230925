extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use log::info;
use std::collections::HashMap;
use std::process::Command;

use crate::{utils, vars};

pub struct InstallBasicSystemSoftware {}
impl TaskTrait for InstallBasicSystemSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let mut system_pack_status = HashMap::new();
        let base_packages = &vars::BASE_PACKAGES.base_packages;
        for i in base_packages {
            if let Ok(v) = utils::install_package(
                i.name.clone(),
                "base_package_script".to_owned(),
                i.script.clone(),
                "/sources/".to_string(),
                "/sources/".to_string(),
            ) {
                system_pack_status.insert(i.script.clone(), v);
            } else {
                system_pack_status.insert(i.script.clone(), false);
            }
        }
        for (k, v) in system_pack_status {
            info!("{} : {}", k, v);
        }

        //按顺序安装软件包
        //在coreutils之前，安装clangd，llvm，rust
        //删除无用文件，保留调试符号
        //
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
