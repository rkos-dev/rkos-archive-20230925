extern crate dagrs;

use dagrs::{EnvVar, Inputval, Retval, TaskTrait};
use log::{error, info};
use std::collections::HashMap;

use crate::{
    utils::{self, InstallInfo, ProgramEndingFlag},
    vars,
};

pub struct InstallRustSupportPackages {}
impl utils::ProgramEndingFlag for InstallRustSupportPackages {}
impl TaskTrait for InstallRustSupportPackages {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();

        let mut rust_support_package_status = HashMap::new();
        let packages = &vars::RUST_SUPPORT_PACKAGES.rust_support_packages;
        for package in packages {
            let package_info = InstallInfo {
                dir_name: package.name.clone(),
                package_name: package.package_name.clone(),
                script_path: vars::BASE_CONFIG.scripts_path.root.clone()
                    + &vars::BASE_CONFIG.scripts_path.build_rust_support_packages,
                script_name: package.script.clone(),
                package_source_path: vars::BASE_CONFIG.path.package_source.clone(),
                package_target_path: vars::BASE_CONFIG.path.package_build.clone(),
            };
            match utils::install_package(package_info, true) {
                Ok(v) => {
                    rust_support_package_status.insert(package.script.clone(), v);
                }
                Err(e) => {
                    rust_support_package_status.insert(package.script.clone(), false);
                    error!(
                        "package {} install failed Err msg: {}",
                        package.name.clone(),
                        e
                    );
                    self.try_set_flag(false);
                }
            }
        }
        for (k, v) in rust_support_package_status {
            info!("{} {}", k, v);
            assert!(v);
        }

        Retval::empty()
    }
}
