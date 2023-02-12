extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use libparted::{Device, Disk};
use nix::mount;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::vars;

// TODO:将所有本阶段合并统一管理

pub trait FinalCheck {
    fn check(&self);
}

fn exec_script(script_path: String) {
    let output = Command::new("bash")
        .arg(script_path)
        .output()
        .expect("error");
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);
}

pub struct PreparingForBuild {}
impl TaskTrait for PreparingForBuild {
    //这里判断各阶段是否完成
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        Retval::new(())
    }
}

impl PreparingSoftware {
    fn preparing_host_software(&self) {
        //需要一个列表用来保存软件包
        //然后调用sudo yum install 来安装这些包
        //捕获输出或者shell的$?来判断是否出错
        // /bin/sh调整为到bash的符号链接或硬链接
        // /usr/bin/yacc 必须是到bison的链接或者是一个执行bison的脚本
        // /usr/bin/awk必须是到gawk的链接
        //执行测试脚本测试环境是否正常
        let packages = &vars::HOST_PACKAGES;
        let mut cmd: String = vars::BASE_CONFIG.host_install_cmd.clone();
        for package in &packages.host_packages {
            cmd += &package;
            cmd += " ";
        }
        println!("{}", &cmd);

        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .status()
            .expect("failed to execute process");
        assert!(output.success());

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

        let base_software = &vars::BASE_PACKAGES.base_packages;
        for i in base_software {
            let output = Command::new("wget")
                .arg("-P")
                .arg("./sources")
                .arg(i.url.as_str())
                .status()
                .expect("");
            print!("{}", output.success());
        }
    }
    fn preparing_cross_compile_software(&self) {}
}

impl FinalCheck for PreparingSoftware {
    fn check(&self) {
        if !Path::new("./version-check.sh").exists() {
            // TODO: 直接打包成.sh
            print!("{}", "warn no script exists");
            return;
        }
        let output = Command::new("bash")
            .arg("-c")
            .arg("chmod +x other_script/version-check.sh && other_script/version-check.sh")
            .status()
            .expect("");
        print!("{}", output.success());
    }
}

pub struct PreparingSoftware {}
impl TaskTrait for PreparingSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //        self.preparing_host_software();
        self.preparing_base_software();
        self.check();

        Retval::new(())
    }
}

pub struct PreparingDisk {}
impl PreparingDisk {
    //在挂载的sdb上创建一个grub bios分区，一般1MB
    //剩余所有空间创建一个ext4分区
    //开机自动挂载文件是/etc/fstab
    //在分区上建立文件系统
    //挂载分区

    fn preparing_new_filesystem(&self) {
        for (_dev_i, mut device) in Device::devices(true).enumerate() {
            println!("Path : {:?}", device.path());
            let disk = Disk::new(&mut device).unwrap();
            for part in disk.parts() {
                println!("{:?}", part.type_get_name());
            }
        }
        println!("over");
    }
    fn preparing_new_partition(&self) {}
    fn preparing_new_dirs(&self) {}
}
impl TaskTrait for PreparingDisk {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        // TODO: 创建分区还在修改中，目前还是手动执行命令
        self.preparing_new_filesystem();
        Retval::new(())
    }
}

struct SettingLfsVariable {}
impl TaskTrait for SettingLfsVariable {
    //设定LFS环境变量并保证在所有时刻都可用
    //可以加入/root/.bash_profile和主目录.bash_profile
    //需要确认/etc/passwd中为每个需要使用LFS变量的用户指定shell为bash
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        Retval::new(())
    }
}

struct PrepareEnvironment {}
impl PrepareEnvironment {
    //创建lfs目录布局
    //添加lfs用户
    //配置lfs环境
    //配置make的线程数
    //创建挂载点并挂载LFS分区
}
impl TaskTrait for PrepareEnvironment {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        // TODO: 换成目录拼接，按照全局目录配置信息
        let target_path = Path::new("./");
        match target_path.exists() {
            true => {}
            false => {}
        }
        // 给target_path 实现nixpath的trait 然后就可以用mount了
        Retval::new(())
    }
}
