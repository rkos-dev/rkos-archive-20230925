extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use log::info;
use std::collections::HashMap;
use std::process::Command;

use crate::{
    utils::{self, InstallInfo},
    vars,
};

pub struct BuildBaseSystem {}
impl TaskTrait for BuildBaseSystem {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let install_packages = TaskWrapper::new(InstallBasicSystemSoftware {}, "Install Packages");
        let mut remove_debug_symbol = TaskWrapper::new(RemoveDebugSymbol {}, "Remove debug symbol");
        let mut clean_up_system = TaskWrapper::new(CleanUpSystem {}, "Clean up system");

        remove_debug_symbol.exec_after(&[&install_packages]);
        clean_up_system.exec_after(&[&remove_debug_symbol]);

        let dag_node = vec![install_packages, remove_debug_symbol, clean_up_system];

        let mut dag = DagEngine::new();
        dag.add_tasks(dag_node);
        assert!(dag.run().unwrap());
        Retval::empty()
    }
}

struct InstallBasicSystemSoftware {}
impl TaskTrait for InstallBasicSystemSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let mut system_pack_status = HashMap::new();
        let base_packages = &vars::BASE_PACKAGES.base_packages;
        for i in base_packages {
            let package_info = InstallInfo {
                package_name: i.name.clone(),
                script_path: "base_package_script".to_owned(),
                script_name: i.script.clone(),
                package_source_path: "/sources/".to_string(),
                package_target_path: "/sources/".to_string(),
            };
            if let Ok(v) = utils::install_package(package_info, true) {
                system_pack_status.insert(i.script.clone(), v);
            } else {
                system_pack_status.insert(i.script.clone(), false);
            }
        }
        for (k, v) in system_pack_status {
            info!("{} : {}", k, v);
        }

        Retval::empty()
    }
}

struct RemoveDebugSymbol {}
impl TaskTrait for RemoveDebugSymbol {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let remove_symbol_path = "other_script/clean_debug_symbol.sh";
        let output = Command::new("/bin/bash")
            .arg(remove_symbol_path)
            .status()
            .expect("error");
        assert!(output.success());
        Retval::empty()
    }
}

struct CleanUpSystem {}
impl TaskTrait for CleanUpSystem {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let remove_symbol_path = "other_script/clean_system.sh";
        let output = Command::new("/bin/bash")
            .arg(remove_symbol_path)
            .status()
            .expect("error");
        Retval::new(output.success())
    }
}
