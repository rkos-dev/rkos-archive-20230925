extern crate dagrs;

use dagrs::{DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use log::info;
use std::collections::HashMap;
use std::process::Command;

use crate::{
    utils::{self, InstallInfo, ProgramEndingFlag},
    vars,
};

pub struct BuildBaseSystem {}
impl utils::ProgramEndingFlag for BuildBaseSystem {}
impl TaskTrait for BuildBaseSystem {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
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

pub struct InstallBasicSystemSoftware {}
impl utils::ProgramEndingFlag for InstallBasicSystemSoftware {}
impl TaskTrait for InstallBasicSystemSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();

        let mut system_pack_status = HashMap::new();
        let base_packages = &vars::BASE_PACKAGES.base_packages;
        for package in base_packages {
            let package_info = InstallInfo {
                //TODO：替换成vars中的常量
                dir_name: package.name.clone(),
                package_name: package.package_name.clone(),
                //script_path: "base_package_script/".to_owned(),
                script_path: vars::BASE_CONFIG.scripts_path.root.clone()
                    + &vars::BASE_CONFIG.scripts_path.build_base_packages,
                script_name: package.script.clone(),
                //package_source_path: "/sources/".to_string(),
                package_source_path: vars::BASE_CONFIG.path.package_source.clone(),
                package_target_path: vars::BASE_CONFIG.path.package_build.clone(),
            };
            if let Ok(v) = utils::install_package(package_info, true) {
                system_pack_status.insert(package.script.clone(), v);
            } else {
                system_pack_status.insert(package.script.clone(), false);
                self.try_set_flag(false);
            }
        }
        for (pack_name, pack_status) in system_pack_status {
            info!("{} : {}", pack_name, pack_status);
        }

        Retval::empty()
    }
}

pub struct RemoveDebugSymbol {}
impl utils::ProgramEndingFlag for RemoveDebugSymbol {}
impl TaskTrait for RemoveDebugSymbol {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        //        let remove_symbol_path = "other_script/clean_debug_symbol.sh";
        let remove_symbol_path =
            vars::BASE_CONFIG.scripts_path.root.clone() + &vars::BASE_CONFIG.scripts_path.clean;

        let status = utils::exec_chroot_script(
            ["remove_debug_symbol.sh"].iter().collect(),
            remove_symbol_path.into(),
        );
        self.try_set_flag(status);
        Retval::empty()
    }
}

pub struct CleanUpSystem {}
impl utils::ProgramEndingFlag for CleanUpSystem {}
impl TaskTrait for CleanUpSystem {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let remove_symbol_path =
            vars::BASE_CONFIG.scripts_path.root.clone() + &vars::BASE_CONFIG.scripts_path.clean;

        let status = utils::exec_chroot_script(
            ["remove_system_trash.sh"].iter().collect(),
            remove_symbol_path.into(),
        );
        self.try_set_flag(status);

        Retval::empty()
    }
}
