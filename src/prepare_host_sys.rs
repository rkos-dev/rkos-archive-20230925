extern crate dagrs;
//extern crate sys_mount;

use dagrs::RunType;
use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, RunScript, TaskTrait, TaskWrapper};
use glob::glob;
use libparted::{Device, Disk, FileSystemType, Partition, PartitionFlag, PartitionType};

use goto::gpoint;
use walkdir::WalkDir;

use cmd_lib::*;
use log::{info, warn};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

use crate::{utils, vars};

pub struct Prepare {}
impl TaskTrait for Prepare {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let backup_bash_profile = TaskWrapper::new(BackupBashProfile {}, "Back up bash profile");
        let prepare_env = TaskWrapper::new(PreparingEnv {}, "PREPARE ENV");
        let mut check_env = TaskWrapper::new(CheckEnv {}, "CHECK ENV");
        let prepare_disk = TaskWrapper::new(PreparingDisk {}, "PREPARE DISK");
        let prepare_software = TaskWrapper::new(PreparingSoftware {}, "PREPARE DISK");

        check_env.exec_after(&[&prepare_env]);

        let mut dag_nodes = vec![prepare_env, check_env, prepare_disk, prepare_software];

        let mut dagrs = DagEngine::new();
        dagrs.add_tasks(dag_nodes);
        assert!(dagrs.run().unwrap());

        Retval::empty()
    }
}

struct BackupBashProfile {}
impl TaskTrait for BackupBashProfile {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let src_path = "/root/.bash_profile";
        let target_path = "/root/.bash_profile.bak";
        fs::rename(src_path, target_path).unwrap();
        Retval::empty()
    }
}

struct PreparingSoftware {}
impl PreparingSoftware {
    //目前使用脚本替代该部分，计划弃用或者完全使用rust
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
    fn preparing_base_software(&self) {
        //创建软件列表，并对应软件下载链接
        //创建临时下载目录
        //给下载目录添加合适权限
        //使用wget下载所有软件包
        //可选的检查所有软件包的正确性

        let all_packages = &vars::ALL_PACKAGES.all_packages;
        let patches = &vars::ALL_PACKAGES.package_patches;
        //        let prepare_download_packages = HashMap::new();
        //        let prepare_download_patches = HashMap::new();
        //        for package in WalkDir::new("/mnt/lfs/sources") {
        //            let package = package.unwrap();
        //            println!("{}", package.path().display());
        //        }
        //
        //        for patches in WalkDir::new("/mnt/lfs/sources/base_patches") {
        //            let patches = patches.unwrap();
        //            println!("{}", patches.path().display());
        //        }

        let mut pack_status = HashMap::new();
        for i in all_packages {
            let mut flag = 0;
            gpoint!['begin:
                if flag>=5{
                    flag=0;
                    pack_status.insert(&i.name,false);
                    break 'begin;
                }

                match utils::download(vars::BASE_CONFIG.package_target_path.clone(), i.url.clone()){
                    Ok(v)=>{
                        match v{
                            true=>{flag=0;pack_status.insert(&i.name,v);break 'begin},
                            false=>{flag+=1;continue 'begin},
                        }
                    },
                    Err(_e)=>{flag+=1;continue 'begin;}
                }
            ];
            //            info!("{:?}", i);
            //            let output = Command::new("wget")
            //                .arg("-P")
            //                .arg("sources")
            //                .arg(i.url.as_str())
            //                .status()
            //                .expect("wget failed");
            //            let status = output.success();
            //            pack_status.insert(&i.name, status);
        }
        for i in patches {
            let mut flag = 0;
            gpoint!['begin:
                if flag>=5{
                    flag=0;
                    pack_status.insert(&i.name,false);
                    break 'begin;
                }

                match utils::download("sources/base_patches".to_owned(), i.url.clone()){

                    Ok(v)=>{
                        match v{
                            true=>{flag=0;pack_status.insert(&i.name,v);break 'begin},
                            false=>{flag+=1;continue 'begin},
                        }
                    },
                    Err(_e)=>{flag+=1;continue 'begin;}
                }
            ];
            //let output = Command::new("wget")
            //    .arg("-P")
            //    .arg("sources")
            //    .arg(i.url.as_str())
            //    .status()
            //    .expect("wget failed");
            //let status = output.success();
            //pack_status.insert(&i.name, status);
        }
        for (k, v) in pack_status {
            info!("{} : {}", k, v);
        }
    }
    fn check(&self) {}
}

impl TaskTrait for PreparingSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.preparing_base_software();
        self.check();
        Retval::new(())
    }
}

struct PreparingEnv {}
impl TaskTrait for PreparingEnv {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let script = RunScript::new("other_script/prepare_env.sh", RunType::SH);
        let res = script.exec(None);
        info!("{:?}", res);
        Retval::empty()
    }
}

struct PreparingDisk {}
impl TaskTrait for PreparingDisk {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let script = RunScript::new("other_script/create_dirs.sh", RunType::SH);
        let res = script.exec(None);
        info!("{:?}", res);
        Retval::empty()
    }
}

struct Env {
    lfs: String,
    lfs_tgt: String,
    path: String,
    lc_all: String,
    config_file: String,
    makeflags: String,
    force_unsafe_configure: i32,
    ninjajobs: i32,
}

struct CheckEnv {}
impl TaskTrait for CheckEnv {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let env = Env {
            lfs: vars::BASE_CONFIG.lfs_env.clone(),
            lfs_tgt: "x86_64-rkos-linux-gnu".to_owned(),
            path: "/tools/bin:/usr/bin".to_owned(),
            lc_all: "POSIX".to_owned(),
            config_file: vars::BASE_CONFIG.lfs_env.clone() + "/usr/share/config.site",
            makeflags: "'-j4'".to_owned(),
            force_unsafe_configure: 1,
            ninjajobs: 4,
        };

        match utils::env_status("LFS".to_owned()) {
            Ok(v) => {
                info!("Get LFS env : {}", v);
            }
            Err(e) => {
                warn!("Nou found LFS env : {}", e);
                let value = &env.lfs;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("LFS_TGT".to_owned()) {
            Ok(v) => {
                info!("Get LFS_TGT env : {}", v);
            }
            Err(e) => {
                warn!("Nou found LFS_TGT env : {}", e);
                let value = &env.lfs_tgt;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("LC_ALL".to_owned()) {
            Ok(v) => {
                info!("Get LC_ALL env : {}", v);
            }
            Err(e) => {
                warn!("Nou found LC_ALL env : {}", e);
                let value = &env.lc_all;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("CONFIG_FILE".to_owned()) {
            Ok(v) => {
                info!("Get CONFIG_FILE env : {}", v);
            }
            Err(e) => {
                warn!("Nou found CONFIG_FILE env : {}", e);
                let value = &env.config_file;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("PATH".to_owned()) {
            Ok(v) => {
                info!("Get PATH env : {}", v);
            }
            Err(e) => {
                warn!("Nou found PATH env : {}", e);
                let value = &env.path;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("MAKEFLAGS".to_owned()) {
            Ok(v) => {
                info!("Get MAKEFLAGS env : {}", v);
            }
            Err(e) => {
                warn!("Nou found MAKEFLAGS env : {}", e);
                let value = &env.makeflags;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("FORCE_UNSAFE_CONFIGURE".to_owned()) {
            Ok(v) => {
                info!("Get FORCE_UNSAFE_CONFIGURE env : {}", v);
            }
            Err(e) => {
                warn!("Nou found FORCE_UNSAFE_CONFIGURE env : {}", e);
                let value = &env.force_unsafe_configure;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }
        match utils::env_status("NINJAJOBS".to_owned()) {
            Ok(v) => {
                info!("Get NINJAJOBS env : {}", v);
            }
            Err(e) => {
                warn!("Nou found NINJAJOBS env : {}", e);
                let value = &env.ninjajobs;
                run_cmd!(echo $value>>/root/.bash_profile).unwrap();
            }
        }

        Retval::empty()
    }
}

//pub struct PreparingDisk {}
//impl PreparingDisk {
//    //在挂载的sdb上创建一个grub bios分区，一般1MB
//    //剩余所有空间创建一个ext4分区
//    //开机自动挂载文件是/etc/fstab
//    //在分区上建立文件系统
//    //挂载分区
//
//    fn preparing_new_filesystem(&self, path: PathBuf) {
//        let mut device = Device::new(path).unwrap();
//        //        for mut device in Device::devices(true) {
//        let mut disk = Disk::new(&mut device).unwrap();
//        for mut part in disk.parts() {
//            println!(
//                "{:?} : {:?} : {:?}",
//                part.name(),
//                part.type_get_name(),
//                part.get_path()
//            );
//        }
//        let fs_type = FileSystemType::get("ext4").expect("no systemtype");
//        println!("{:?}", fs_type.name());
//        assert_eq!(1, 0); //TODO:后续部分不能在本机上测试
//        let mut new_part = Partition::new(
//            &disk,
//            PartitionType::PED_PARTITION_LOGICAL,
//            FileSystemType::get("ext4").as_ref(),
//            0,
//            128,
//        )
//        .unwrap();
//        new_part.set_flag(PartitionFlag::PED_PARTITION_BOOT, true);
//        let constraint = new_part.get_geom();
//        let constraint = match constraint.exact() {
//            Some(v) => v,
//            None => panic!("err"),
//        };
//        //{
//        //   Some(v) => v,
//        //  None => panic!("no constraint"),
//        //};
//        disk.add_partition(&mut new_part, &constraint);
//        //       }
//        println!("over");
//    }
//    fn preparing_new_partition(&self) {}
//    fn preparing_new_dirs(&self) {}
//}
//impl TaskTrait for PreparingDisk {
//    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
//        // TODO: 创建分区修改好了，最后再测试，目前还是手动执行命令
//        let path: PathBuf = ["/dev"].iter().collect();
//        self.preparing_new_filesystem(path);
//        Retval::new(())
//    }
//}
//
//struct SettingLfsVariable {}
//impl TaskTrait for SettingLfsVariable {
//    //设定LFS环境变量并保证在所有时刻都可用
//    //可以加入/root/.bash_profile和主目录.bash_profile
//    //需要确认/etc/passwd中为每个需要使用LFS变量的用户指定shell为bash
//    //TODO:计划改为软件运行前用户手动设定
//    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
//        Retval::new(())
//    }
//}
//
//struct PrepareEnvironment {}
//impl PrepareEnvironment {
//    //创建lfs目录布局
//    //添加lfs用户
//    //配置lfs环境
//    //配置make的线程数
//    //创建挂载点并挂载LFS分区
//    //已经通过脚本设置好了
//}
//impl TaskTrait for PrepareEnvironment {
//    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
//        // TODO: 换成目录拼接，按照全局目录配置信息
//        let target_path = Path::new("./");
//        match target_path.exists() {
//            true => {}
//            false => {}
//        }
//        // 给target_path 实现nixpath的trait 然后就可以用mount了
//        Retval::new(())
//    }
//}
