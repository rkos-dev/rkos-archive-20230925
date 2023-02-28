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

        compile_toolchains.exec_after(&[&check_system_env]);
        enter_chroot.exec_after(&[&compile_toolchains]);
        install_other_packages.exec_after(&[&enter_chroot]);
        clean_system.exec_after(&[&compile_toolchains, &enter_chroot, &install_other_packages]);

        let dag_nodes = vec![
            check_system_env,
            compile_toolchains,
            enter_chroot,
            install_other_packages,
            clean_system,
        ];
        let mut dagrs = DagEngine::new();
        dagrs.add_tasks(dag_nodes);
        assert!(dagrs.run().unwrap());
        Retval::empty()
    }
}

//检查lfs环境变量
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

//编译交叉工具链以及chroot前的其他工具
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
            let res = utils::install_package(pack_i, false);
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
            let res = utils::install_package(pack_i, false);
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

        Retval::empty()
    }
}

//准备chroot后的环境
struct AfterChroot {}
impl TaskTrait for AfterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //       let create_script_path: PathBuf = ["chroot_scripts", "after_chroot_create_dirs.sh"]
        //           .iter()
        //           .collect();
        let status = utils::exec_chroot_script(
            ["after_chroot_create_dirs.sh"].iter().collect(),
            ["/chroot_scripts"].iter().collect(),
        );
        //        let status = exec_chroot_script(create_script_path);
        assert!(status);
        let status = utils::exec_chroot_script(
            ["after_chroot_create_files.sh"].iter().collect(),
            ["/chroot_scripts"].iter().collect(),
        );
        assert!(status);

        //let create_script_path: PathBuf = ["chroot_scripts", "after_chroot_create_files.sh"]
        //    .iter()
        //    .collect();
        //let status = exec_chroot_script(create_script_path);
        //assert!(status);
        Retval::empty()
    }
}

// chroot之后安装临时工具
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
            let res = utils::install_package(pack_i, true);
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

// 进入chroot环境，并准备好第一次进入chroot环境后的配置
struct EnterChroot {}
impl TaskTrait for EnterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let chroot = TaskWrapper::new(utils::EnterFakeroot {}, "Chroot");
        let mut after_chroot = TaskWrapper::new(AfterChroot {}, "After chroot config");

        after_chroot.exec_after(&[&chroot]);

        let dag_nodes = vec![chroot, after_chroot];
        let mut dag = DagEngine::new();

        dag.add_tasks(dag_nodes);

        assert!(dag.run().unwrap());

        //修改临时环境目录的所有者
        //挂载内核文件系统
        //移动所有文件
        //进入chroot环境
        // - 本体chroot
        // - set 目录到/
        // - 删除所有env
        // - 重新设定HOME PATH
        Retval::empty()
    }
}

//清理环境临时工具等
struct CleanUpAndSaveTempSystem {}
impl TaskTrait for CleanUpAndSaveTempSystem {
    //清理临时工具
    //备份系统
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
