extern crate dagrs;
//extern crate sys_mount;

use dagrs::{EnvVar, Inputval, Retval, TaskTrait};

#[allow(unused)]
use libparted::{Device, Disk, FileSystemType, Partition, PartitionFlag, PartitionType};

#[allow(unused)]
use walkdir::WalkDir;

use cmd_lib::*;
use log::{error, info, warn};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::process::Command;

use crate::utils::ProgramEndingFlag;
use crate::{utils, vars};

#[allow(unused)]
pub struct CleanOldConfig {}
impl TaskTrait for CleanOldConfig {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //TODO: 删除环境变量配置
        //卸载目录
        //删除目录配置
        //
        Retval::empty()
    }
}
impl utils::ProgramEndingFlag for CleanOldConfig {}

//TODO:添加全局重置脚本
#[allow(unused)]
pub struct ResetAllSystemConfig {}
impl TaskTrait for ResetAllSystemConfig {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        // reset bash profile
        // reset target dirs
        // reset system auto mount
        //
        Retval::empty()
    }
}

pub struct PreparingSoftware {}
impl utils::ProgramEndingFlag for PreparingSoftware {}
impl PreparingSoftware {
    /// 下载软件包和软件包补丁
    //    #[allow(unused)]
    //    fn download_packages(&self) {
    //        //        let all_packages = &vars::ALL_PACKAGES.all_packages;
    //        //        let all_patches = &vars::ALL_PACKAGES.package_patches;
    //        let all_packages = &vars::PACKAGES.package_info;
    //        let all_patches = &vars::PACKAGES.package_patches;
    //
    //        let mut dls = Vec::new();
    //
    //        for package in all_packages {
    //            dls.push(package.url.as_str());
    //            //TODO:!!!!!
    //        }
    //        for patch in all_patches {
    //            dls.push(patch.url.as_str());
    //        }
    //        info!("{:?}", dls);
    //        let status = utils::new_downlaod("./source".to_string(), &dls[..]);
    //        match status {
    //            Ok(v) => info!("ok"),
    //            Err(e) => error!("error {:?}", e),
    //        }
    //    }
    fn preparing_base_software(&self) {
        //软件包和补丁列表
        //        let all_packages = &vars::ALL_PACKAGES.all_packages;
        //        let patches = &vars::ALL_PACKAGES.package_patches;

        let all_packages = &vars::PACKAGES.package_info;
        let all_patches = &vars::PACKAGES.package_patches;

        //软件包下载状态记录
        let mut pack_status = HashMap::new();

        for package in all_packages {
            //
            let mut try_download_times = 0;

            //检查是否已经存在数据,原本存在就不需要下载
            if utils::check_download_before(
                &package.package_name,
                &vars::BASE_CONFIG.path.package_build,
            ) {
                continue;
            }

            while try_download_times < 5 {
                utils::delete_failed_download_pack(
                    &package.package_name,
                    &vars::BASE_CONFIG.path.package_build,
                );
                match utils::download(
                    vars::BASE_CONFIG.path.package_build.clone(),
                    package.url.clone(),
                ) {
                    Ok(v) => {
                        match v {
                            true => {
                                //下载成功
                                try_download_times = 0;
                                pack_status.insert(&package.package_name, v);
                                break;
                            }
                            false => {
                                //下载失败，重试
                                try_download_times += 1;
                                continue;
                            }
                        }
                    }
                    Err(_e) => {
                        //下载命令执行失败，重试
                        //FIXME:下载命令执行失败应该print命令然后终止
                        try_download_times += 1;
                        continue;
                    }
                }
            }
            if try_download_times == 5 {
                pack_status.insert(&package.package_name, false);
            }
        }

        //下载补丁
        for patch in all_patches {
            //重试次数
            let mut try_download_times = 0;

            //检查是否存在
            //FIXME:目录不是补丁目录，需要调整
            if utils::check_download_before(
                &patch.patch_name,
                &(vars::BASE_CONFIG.path.package_source.clone()
                    + &vars::BASE_CONFIG.path.package_patches),
            ) {
                continue;
            }

            while try_download_times < 5 {
                //大于5次，下载失败

                //TODO:调整成由var配置的部分
                //并且添加对patches的存在性判断
                match utils::download(
                    vars::BASE_CONFIG.path.package_source.clone()
                        + &vars::BASE_CONFIG.path.package_patches,
                    patch.url.clone(),
                ) {
                    Ok(v) => match v {
                        true => {
                            try_download_times = 0;
                            pack_status.insert(&patch.patch_name, v);
                            break;
                        }
                        false => {
                            try_download_times += 1;
                            continue;
                        }
                    },
                    Err(_e) => {
                        try_download_times += 1;
                        continue;
                    }
                }
            }
            if try_download_times == 5 {
                pack_status.insert(&patch.patch_name, false);
            }
        }

        for (package_name, download_status) in pack_status {
            info!("{} : {}", package_name, download_status);
            self.try_set_flag(download_status);
        }
    }
}

impl TaskTrait for PreparingSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        self.preparing_base_software();
        //self.download_packages();
        Retval::empty()
    }
}

//备份环境变量配置文件
pub struct BackupBashProfile {}
impl utils::ProgramEndingFlag for BackupBashProfile {}
impl TaskTrait for BackupBashProfile {
    fn run(&self, mut input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        match input.get::<bool>(0).unwrap() {
            //无需备份返回true
            true => Retval::new(true),
            false => {
                //需要备份返回false
                let src_path = "/root/.bash_profile";
                let target_path = "/root/.bash_profile.bak";
                match fs::rename(src_path, target_path) {
                    Ok(_v) => {
                        info!("Back up bash profile success");
                    }
                    Err(e) => {
                        error!("Back up bash profile failed {}", e);
                        self.try_set_flag(false);
                    }
                }

                Retval::new(false)
            }
        }
    }
}

// TODO 完全替换成rust实现
pub struct PreparingEnv {}
impl utils::ProgramEndingFlag for PreparingEnv {}
impl PreparingEnv {
    #[allow(unused)]
    fn prepare_env(&self) {
        //        let packages = &vars::HOST_PACKAGES;
        //        let mut cmd: String = vars::BASE_CONFIG.host_install_cmd.clone();
        //        for package in &packages.host_packages {
        //            cmd += &package;
        //            cmd += " ";
        //        }
        //        println!("{}", &cmd);
        //
        //        let output = Command::new("bash")
        //            .arg("-c")
        //            .arg(cmd)
        //            .status()
        //            .expect("failed to execute process");
        //        assert!(output.success());

        let mut link_list = HashMap::new();
        link_list.insert("sh", "bash");
        link_list.insert("yacc", "bison");
        link_list.insert("awk", "gawk");
        fs::remove_file("/usr/bin/sh").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        fs::remove_file("/usr/bin/yacc").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        fs::remove_file("/usr/bin/awk").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        for link in &link_list {
            // TODO: 换成unix::fs::symlink
            let cmd = format!("sudo ln -s /usr/bin/{} /usr/bin/{}", &link.1, &link.0);
            let create_link_res = Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .status()
                .expect("failed to create link");
            if !create_link_res.success() {
                println!("failed to create link");
            }
        }
    }
    #[allow(unused)]
    fn check() {}
}

//准备宿主机目标安装分区的目录
//FIXME:确保每一次运行的时候都能检查是否mount了分区
pub struct PreparingDirs {}
impl utils::ProgramEndingFlag for PreparingDirs {}
impl TaskTrait for PreparingDirs {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path = vars::BASE_CONFIG.scripts_path.root.clone()
            + &vars::BASE_CONFIG.scripts_path.prepare
            + "prepare_host_dirs.sh";
        //        let script = RunScript::new("other_script/create_dirs.sh", RunType::SH);
        //        let script = RunScript::new(&script_path, RunType::SH);

        let stdout_file = File::create("/root/prepare.log").unwrap();

        let stderr_file = stdout_file.try_clone().unwrap();

        match Command::new("/bin/bash")
            .arg("-e")
            .arg(script_path)
            .stdout(stdout_file)
            .stderr(stderr_file)
            .status()
        {
            Ok(_v) => (),
            Err(_e) => self.try_set_flag(false),
        };

        //        let res = script.exec(None);
        Retval::empty()
    }
}

pub struct MountTargetPart {}
impl utils::ProgramEndingFlag for MountTargetPart {}
impl TaskTrait for MountTargetPart {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        Retval::empty()
    }
}

//检查并配置环境变量
pub struct CheckEnv {}
impl utils::ProgramEndingFlag for CheckEnv {}
impl TaskTrait for CheckEnv {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        //从配置文件中读出环境变量配置，设定到本进程上。
        for env in &vars::BASE_CONFIG.envs {
            match utils::env_status(env.name.clone()) {
                Ok(v) => {
                    if v.clone() == env.value.clone() {
                        info!("Get {} env : {}", env.name.clone(), v.clone());
                    } else {
                        env::set_var(env.name.clone(), env.value.clone());
                        warn!(
                            "Get {} env , reset value from {} to {}",
                            env.name.clone(),
                            v.clone(),
                            env.value.clone()
                        );
                    }
                }
                Err(_e) => {
                    warn!(
                        "Nou found env {} , set new value {}",
                        env.name.clone(),
                        env.value.clone()
                    );
                    env::set_var(env.name.clone(), env.value.clone());
                }
            }
        }
        Retval::new(true)
    }
}
