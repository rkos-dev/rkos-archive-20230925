extern crate dagrs;

use crate::utils;
use crate::vars;
use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use glob::glob;
use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::os::unix::fs::chroot;
use std::path::PathBuf;
use std::process::Command;

fn exec_script(script_path: PathBuf, dir: PathBuf) -> bool {
    let abs_path = fs::canonicalize(dir.as_path()).unwrap();
    let filename = match script_path.to_str() {
        Some(v) => v,
        None => panic!("cannot turn to str"),
    };
    let output = Command::new("/bin/bash")
        .current_dir(abs_path)
        .arg(filename)
        .status()
        .expect("error");
    output.success()
}

fn exec_chroot_script(script_path: PathBuf) -> bool {
    let output = Command::new("ls")
        .env_clear()
        .env("PATH", "/bin")
        .env("PATH", "/sbin")
        .env("HOME", "/root")
        .status()
        .expect("error");
    output.success()
}

pub struct CompileTempPackages {}
impl TaskTrait for CompileTempPackages {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let check_system_env = TaskWrapper::new(CheckEnv {}, "Check Env");

        let mut compile_toolchains =
            TaskWrapper::new(CompilingCrossToolChain {}, "Compile Toolchains");

        let mut enter_chroot = TaskWrapper::new(EnterChroot {}, "Enter Chroot");

        let mut install_other_packages =
            TaskWrapper::new(AfterChrootInstall {}, "Install other packages");

        let mut clean_system = TaskWrapper::new(CleanUpAndSaveTempSystem {}, "Clean up");

        let dag_nodes = vec![
            check_system_env,
            compile_toolchains,
            enter_chroot,
            install_other_packages,
            clean_system,
        ];

        compile_toolchains.exec_after(&[&check_system_env]);
        enter_chroot.exec_after(&[&compile_toolchains]);
        install_other_packages.exec_after(&[&enter_chroot]);
        clean_system.exec_after(&[&compile_toolchains, &enter_chroot, &install_other_packages]);

        let mut dagrs = DagEngine::new();
        dagrs.add_tasks(dag_nodes);
        assert!(dagrs.run().unwrap());
        Retval::empty()
    }
}

pub struct CheckEnv {}
impl TaskTrait for CheckEnv {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let lfs_env = "LFS";
        let status = match env::var(lfs_env) {
            Ok(v) => return Retval::empty(),
            Err(e) => panic!("LFS env not found"),
        };
    }
}

pub struct CompilingCrossToolChain {}
impl CompilingCrossToolChain {
    fn check_system_env(&self) -> Result<String, env::VarError> {
        let lfs_env = "LFS";
        let status = match env::var(lfs_env) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(e),
        };
    }

    fn before_chroot_install_packages(&self, lfs_env: String) -> Result<(), std::io::Error> {
        let mut package_install_status = HashMap::new();
        let cross_compile_toolchains = &vars::CROSS_COMPILE_PACKAGES.cross_compile_toolchains;
        let cross_compile_packages = &vars::CROSS_COMPILE_PACKAGES.cross_compile_packages;
        info!(
            "{:?} {:?}",
            &cross_compile_toolchains, &cross_compile_packages
        );
        for i in cross_compile_toolchains {
            let pack_i = utils::InstallInfo {
                package_name: i.name.clone(),
                script_name: i.script.clone(),
                script_path: "cross_compile_script/".to_owned(),
                package_source_path: "/mnt/lfs/sources/".to_owned(),
                package_target_path: "/mnt/lfs/sources/".to_owned(),
            };
            //            let res = utils::install_package(
            //                i.name.clone(),
            //                "cross_compile_script/".to_owned(),
            //                i.script.clone(),
            //                "/mnt/lfs/sources/".to_owned(),
            //                "/mnt/lfs/sources/".to_owned(),
            //            );
            let res = utils::install_package(pack_i);
            match res {
                Ok(v) => package_install_status.insert(i.script.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(i.script.clone(), false);
                    break;
                }
            };
        }
        for i in cross_compile_packages {
            //            let res = utils::install_package(
            //                i.name.clone(),
            //                "cross_compile_script/".to_owned(),
            //                i.script.clone(),
            //                "/mnt/lfs/sources/".to_owned(),
            //                "/mnt/lfs/sources".to_owned(),
            //            );
            let pack_i = utils::InstallInfo {
                package_name: i.name.clone(),
                script_name: i.script.clone(),
                script_path: "cross_compile_script/".to_owned(),
                package_source_path: "/mnt/lfs/sources/".to_owned(),
                package_target_path: "/mnt/lfs/sources/".to_owned(),
            };
            let res = utils::install_package(pack_i);
            match res {
                Ok(v) => package_install_status.insert(i.script.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(i.script.clone(), false);
                    break;
                }
            };
        }

        for (k, v) in package_install_status {
            info!("{} : {}", k, v);
        }

        Ok(())
    }

    fn check_data(&self, package_name: String) {
        let cross_compile_toolchains = &vars::CROSS_COMPILE_PACKAGES.cross_compile_toolchains;
        let cross_compile_packages = &vars::CROSS_COMPILE_PACKAGES.cross_compile_packages;
        let after_chroot_packages = &vars::CROSS_COMPILE_PACKAGES.after_chroot_packages;
        let script_path = "cross_compile_script";
        let sources_path = "sources";
    }

    fn delete_package(&self, package_path: PathBuf) -> std::io::Result<()> {
        fs::remove_dir_all(package_path)?;
        Ok(())
    }
}

impl TaskTrait for CompilingCrossToolChain {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let lfs_env = "/mnt/lfs".to_string();
        info!("start install {}", lfs_env);

        //        self.install_packages(lfs_env).unwrap();
        self.before_chroot_install_packages(lfs_env).unwrap();

        Retval::new(())
    }
}

struct AfterChrootInstall {}
impl AfterChrootInstall {
    fn after_chroot_install_packages(&self) -> Result<(), std::io::Error> {
        let mut package_install_status = HashMap::new();
        let after_chroot_packages = &vars::CROSS_COMPILE_PACKAGES.after_chroot_packages;
        info!("{:?}", after_chroot_packages);
        for i in after_chroot_packages {
            let pack_i = utils::InstallInfo {
                package_name: i.name.clone(),
                script_name: i.script.clone(),
                script_path: "cross_compile_script/".to_owned(),
                package_source_path: "/sources/".to_owned(),
                package_target_path: "/sources/".to_owned(),
            };
            let res = utils::install_package(pack_i);
            match res {
                Ok(v) => package_install_status.insert(i.script.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(i.script.clone(), false)
                }
            };
        }
        Ok(())
    }
}
impl TaskTrait for AfterChrootInstall {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        if let Ok(v) = self.after_chroot_install_packages() {
            return Retval::empty();
        } else {
            panic!("Cannot installl packages in chroot env");
        }
    }
}

struct EnterChroot {}
impl EnterChroot {
    fn prepare_chroot_chmod(&self) -> bool {
        let chmod_script_path = "chroot_scripts/chown.sh";
        let output = Command::new("/bin/bash")
            .arg(chmod_script_path)
            .status()
            .expect("error");
        output.success()
    }
    fn prepare_virt_fsys(&self) -> bool {
        let prepare_virtual_fsys = "chroot_scripts/prepare_vir_filesystem.sh";
        let output = Command::new("/bin/bash")
            .arg(prepare_virtual_fsys)
            .status()
            .expect("error");
        output.success()
    }

    fn enter_chroot(&self) -> std::io::Result<()> {
        chroot("/mnt/lfs")?;
        std::env::set_current_dir("/")?;
        Ok(())
    }
    fn create_dirs(&self) -> bool {
        let create_script_path: PathBuf = ["chroot_scripts", "after_chroot_create_dirs.sh"]
            .iter()
            .collect();
        let status = exec_chroot_script(create_script_path);
        status
    }
    fn create_files(&self) -> bool {
        let create_script_path: PathBuf = ["chroot_scripts", "after_chroot_create_files.sh"]
            .iter()
            .collect();
        let status = exec_chroot_script(create_script_path);
        status
    }
}
impl TaskTrait for EnterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let status = self.prepare_chroot_chmod();
        assert!(status);
        let status = self.prepare_virt_fsys();
        assert!(status);
        self.enter_chroot();
        let status = self.create_dirs();
        assert!(status);
        let status = self.create_files();
        assert!(status);

        //修改临时环境目录的所有者
        //挂载内核文件系统
        //移动所有文件
        //进入chroot环境
        // - 本体chroot
        // - set 目录到/
        // - 删除所有env
        // - 重新设定HOME PATH
        Retval::new(())
    }
}

struct CleanUpAndSaveTempSystem {}
impl TaskTrait for CleanUpAndSaveTempSystem {
    //清理临时工具
    //备份系统
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
