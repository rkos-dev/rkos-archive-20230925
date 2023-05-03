extern crate dagrs;

use crate::utils;
use crate::utils::ProgramEndingFlag;
use crate::vars;
use clap::Parser;
use dagrs::{EnvVar, Inputval, Retval, TaskTrait};
use log::{error, info};
use std::collections::HashMap;
use std::error::Error;

pub struct PackageInput {}
impl utils::ProgramEndingFlag for PackageInput {}
impl TaskTrait for PackageInput {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let cli = vars::Cli::parse();
        let user_input = cli.package_name;
        Retval::new(user_input)
    }
}

//编译交叉工具链以及chroot前的其他工具
pub struct CompilingCrossToolChain {}
impl utils::ProgramEndingFlag for CompilingCrossToolChain {}
impl CompilingCrossToolChain {
    fn before_chroot_install_packages(&self) -> Result<(), Box<dyn Error>> {
        let mut package_install_status = HashMap::new();
        let cross_compile_toolchains = &vars::PACKAGES.install_info.cross_compile_toolchains;
        let cross_compile_packages = &vars::PACKAGES.install_info.cross_compile_packages;

        //        let cross_compile_toolchains = &vars::CROSS_COMPILE_PACKAGES.cross_compile_toolchains;
        //        let cross_compile_packages = &vars::CROSS_COMPILE_PACKAGES.cross_compile_packages;
        info!(
            "waiting for {:?} {:?}",
            &cross_compile_toolchains, &cross_compile_packages
        );

        //安装临时工具链
        for package in cross_compile_toolchains {
            let pack_install_info = utils::InstallInfo {
                dir_name: package.package_name.clone(),
                package_name: package.package_name.clone(),
                script_name: package.script_name.clone(),

                script_path: vars::BASE_CONFIG.scripts_path.root.clone()
                    + &vars::BASE_CONFIG.scripts_path.build_temp_toolchains,
                package_source_path: vars::BASE_CONFIG.path.install_path.clone()
                    + &vars::BASE_CONFIG.path.package_source,
                package_target_path: vars::BASE_CONFIG.path.install_path.clone()
                    + &vars::BASE_CONFIG.path.package_build,
            };

            let res = utils::install_package(pack_install_info, false);
            match res {
                Ok(v) => package_install_status.insert(package.script_name.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(package.script_name.clone(), false);
                    return Err(format!("Failed install package {}", &package.package_name).into());
                }
            };
        }
        //安装临时工具
        for package in cross_compile_packages {
            let pack_install_info = utils::InstallInfo {
                dir_name: package.package_name.clone(),
                package_name: package.package_name.clone(),
                script_name: package.script_name.clone(),
                script_path: vars::BASE_CONFIG.scripts_path.root.clone()
                    + &vars::BASE_CONFIG.scripts_path.build_temp_toolchains,
                //                script_path: "cross_compile_script/".to_owned(),
                package_source_path: vars::BASE_CONFIG.path.install_path.clone()
                    + &vars::BASE_CONFIG.path.package_source,
                //                package_source_path: "/mnt/lfs/sources/".to_owned(),
                package_target_path: vars::BASE_CONFIG.path.install_path.clone()
                    + &vars::BASE_CONFIG.path.package_build,
                //                package_target_path: "/mnt/lfs/sources/".to_owned(),
            };
            let res = utils::install_package(pack_install_info, false);
            match res {
                Ok(v) => package_install_status.insert(package.script_name.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(package.script_name.clone(), false);
                    return Err(format!("Failed install package {}", &package.package_name).into());
                }
            };
        }

        for (pack_name, pack_status) in package_install_status {
            info!("{} : {}", pack_name, pack_status);
            //FIXME:下面一句没必要，和上面的逻辑还有重复的地方
            assert!(pack_status);
        }

        Ok(())
    }

    //TODO:检查安装状态
    #[allow(unused)]
    fn check_data(&self, package_name: String) {
        let cross_compile_toolchains = &vars::PACKAGES.install_info.cross_compile_toolchains;
        let cross_compile_packages = &vars::PACKAGES.install_info.cross_compile_packages;
        let after_chroot_packages = &vars::PACKAGES.install_info.after_chroot_packages;
        let script_path = "cross_compile_script";
        let sources_path = "sources";
    }
}
impl TaskTrait for CompilingCrossToolChain {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        match self.before_chroot_install_packages() {
            Ok(_v) => {
                info!("cross compile toolchain install success!");
            }
            Err(_e) => self.try_set_flag(false),
        }

        Retval::empty()
    }
}

//准备chroot后的环境

pub struct AfterChroot {}
impl utils::ProgramEndingFlag for AfterChroot {}
impl TaskTrait for AfterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let status = utils::exec_chroot_script(
            ["create_dirs.sh"].iter().collect(),
            [
                &vars::BASE_CONFIG.scripts_path.root,
                &vars::BASE_CONFIG.scripts_path.chroot,
            ]
            .iter()
            .collect(),
        );

        self.try_set_flag(status);
        let status = utils::exec_chroot_script(
            ["create_files.sh"].iter().collect(),
            [
                &vars::BASE_CONFIG.scripts_path.root,
                &vars::BASE_CONFIG.scripts_path.chroot,
            ]
            .iter()
            .collect(),
        );
        self.try_set_flag(status);

        Retval::empty()
    }
}

// chroot之后安装临时工具
pub struct AfterChrootInstall {}
impl utils::ProgramEndingFlag for AfterChrootInstall {}
impl AfterChrootInstall {
    fn after_chroot_install_packages(&self) -> Result<(), Box<dyn Error>> {
        let mut package_install_status = HashMap::new();
        let after_chroot_packages = &vars::PACKAGES.install_info.after_chroot_packages;
        info!("{:?}", after_chroot_packages);
        for packages in after_chroot_packages {
            let pack_build_info = utils::InstallInfo {
                dir_name: packages.package_name.clone(),
                package_name: packages.package_name.clone(),
                script_name: packages.script_name.clone(),
                //                script_path: "cross_compile_script/".to_owned(),
                script_path: vars::BASE_CONFIG.scripts_path.root.clone()
                    + &vars::BASE_CONFIG.scripts_path.build_temp_toolchains,
                //                package_source_path: "/sources/".to_owned(),
                package_source_path: vars::BASE_CONFIG.path.package_source.clone(),
                package_target_path: vars::BASE_CONFIG.path.package_build.clone(),
            };
            let res = utils::install_package(pack_build_info, true);
            match res {
                Ok(v) => package_install_status.insert(packages.script_name.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(packages.script_name.clone(), false);
                    return Err(format!("Failed install package {}", &packages.package_name).into());
                }
            };
        }
        for (k, v) in package_install_status {
            info!("{} {}", k, v);
            assert!(v);
        }
        Ok(())
    }
}

//安装其他的临时工具链
impl TaskTrait for AfterChrootInstall {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        if self.after_chroot_install_packages().is_ok() {
            Retval::empty()
        } else {
            self.try_set_flag(false);
            panic!("Cannot installl packages in chroot env");
        }
    }
}

//清理环境临时工具等
pub struct CleanUpAndSaveTempSystem {}
impl utils::ProgramEndingFlag for CleanUpAndSaveTempSystem {}
impl TaskTrait for CleanUpAndSaveTempSystem {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let status = utils::exec_chroot_script(
            ["remove_temp_file.sh"].iter().collect(),
            //            ["/chroot_scripts"].iter().collect(),
            [
                &vars::BASE_CONFIG.scripts_path.root,
                &vars::BASE_CONFIG.scripts_path.chroot,
            ]
            .iter()
            .collect(),
        );

        self.try_set_flag(status);
        Retval::empty()
    }
}
