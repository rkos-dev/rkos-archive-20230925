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

pub struct CompilingCrossToolChain {}
impl CompilingCrossToolChain {
    fn check_system_env(&self) -> Result<String, env::VarError> {
        let lfs_env = "LFS";
        let status = match env::var(lfs_env) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(e),
        };
    }

    fn new_install_packages(&self, lfs_env: String) -> Result<(), std::io::Error> {
        let mut package_install_status = HashMap::new();
        let cross_compile_toolchains = &vars::CROSS_COMPILE_PACKAGES.cross_compile_toolchains;
        let cross_compile_packages = &vars::CROSS_COMPILE_PACKAGES.cross_compile_packages;
        let after_chroot_packages = &vars::CROSS_COMPILE_PACKAGES.after_chroot_packages;
        info!(
            "{:?} {:?} {:?}",
            &cross_compile_toolchains, &cross_compile_packages, &after_chroot_packages
        );
        for i in cross_compile_toolchains {
            let res = utils::install_package(
                i.name.clone(),
                "cross_compile_script/".to_owned(),
                i.script.clone(),
                "sources/".to_owned(),
                "sources/".to_owned(),
            );
            match res {
                Ok(v) => package_install_status.insert(i.script.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(i.script.clone(), false);
                    return Ok(());
                }
            };
        }
        for i in cross_compile_packages {
            let res = utils::install_package(
                i.name.clone(),
                "cross_compile_script/".to_owned(),
                i.script.clone(),
                "sources/".to_owned(),
                "sources/".to_owned(),
            );
            match res {
                Ok(v) => package_install_status.insert(i.script.clone(), v),
                Err(e) => {
                    error!("{:?}", e);
                    package_install_status.insert(i.script.clone(), false);
                    return Ok(());
                }
            };
        }
        //        for i in after_chroot_packages {
        //            let res = utils::install_package(
        //                i.name.clone(),
        //                "cross_compile_script/".to_owned(),
        //                i.script.clone(),
        //                "sources/".to_owned(),
        //                "sources/".to_owned(),
        //            );
        //            match res {
        //                Ok(v) => package_install_status.insert(i.script.clone(), v),
        //                Err(e) => {
        //                    error!("{:?}", e);
        //                    package_install_status.insert(i.script.clone(), false)
        //                }
        //            };
        //        }
        for (k, v) in package_install_status {
            info!("{} : {}", k, v);
        }

        Ok(())
    }

    fn check_data(&self, package_name: String) {
        //检测命令的状态就可以
    }

    fn delete_package(&self, package_path: PathBuf) -> std::io::Result<()> {
        fs::remove_dir_all(package_path)?;
        Ok(())
    }
}
impl TaskTrait for CompilingCrossToolChain {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //首先确认LFS环境变量，bash是否是正在用的shell，sh是否指向bash，awk是否指向gawk，yacc是否指向bison
        //获取所有安装软件的包名按顺序插入列表
        //判断软件包是否存在，如果存在
        //解压缩软件包，并push到包目录，然后执行安装脚本
        //判断输出是否正常，软件包安装是否正常
        //删除软件包
        //否则下载软件包然后重复上述过程
        //        let lfs_env = match self.check_system_env() {
        //            Ok(v) => v,
        //            Err(e) => {
        //                error!("LFS_ENV ERROR : {}", e);
        //                //TODO:使用错误常量来定义错误
        //                std::process::exit(1);
        //            }
        //        };

        let lfs_env = "/mnt/lfs".to_string();
        info!("start install {}", lfs_env);

        //        self.install_packages(lfs_env).unwrap();
        self.new_install_packages(lfs_env).unwrap();

        Retval::new(())
    }
}

struct EnterChroot {}
impl EnterChroot {
    fn prepare_chroot_chmod(&self) -> bool {
        let chmod_script_path = "enter_chroot_scripts/chown.sh";
        let output = Command::new("/bin/bash")
            .arg(chmod_script_path)
            .status()
            .expect("error");
        output.success()
    }
    fn prepare_virt_fsys(&self) -> bool {
        let prepare_virtual_fsys = "enter_chroot_scripts/prepare_vir_filesystem.sh";
        let output = Command::new("/bin/bash")
            .arg(prepare_virtual_fsys)
            .status()
            .expect("error");
        output.success()
        //脚本来实现
    }
    fn enter_chroot(&self) -> std::io::Result<()> {
        chroot("/mnt/lfs")?;
        std::env::set_current_dir("/")?;
        Ok(())
    }
    fn create_path(&self) -> bool {
        let created_script_path: PathBuf = ["enter_chroot_scripts", "created_script.sh"]
            .iter()
            .collect();
        let status = exec_chroot_script(created_script_path);
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
        let status = self.create_path();
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

struct CompileTempPackages {}
impl TaskTrait for CompileTempPackages {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //获取所有安装软件的包名按顺序插入列表
        //判断软件包是否存在，如果存在
        //解压缩软件包，并push到包目录，然后执行安装脚本
        //判断输出是否正常，软件包安装是否正常
        //删除软件包
        //否则下载软件包然后重复上述过程

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
