extern crate dagrs;

use crate::vars;
use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use glob::glob;
use std::env;
use std::fs;
use std::fs::File;
use std::os::unix::fs::chroot;
use std::path::PathBuf;
use std::process::Command;

fn exec_script(script_path: PathBuf) {
    let output = Command::new("/bin/bash")
        .arg(script_path)
        .output()
        .expect("error");
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);
}

fn exec_chroot_script(script_path: PathBuf) {
    let output = Command::new("ls")
        .env_clear()
        .env("PATH", "/bin")
        .env("PATH", "/sbin")
        .spawn()
        .expect("error");
}

pub struct CompilingCrossToolChain {}
impl CompilingCrossToolChain {
    fn check_system_env(&self) -> Result<String, env::VarError> {
        let lfs_env = "LFS";
        match env::var(lfs_env) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn install_packages(&self, lfs_env: String) -> Result<(), std::io::Error> {
        let cross_compile_package = &vars::CROSS_COMPILE_PACKAGES.cross_compile_toolchains;
        for i in cross_compile_package {
            let package_path = match glob(&("sources/".to_owned() + &i + "*"))
                .unwrap()
                .filter_map(Result::ok)
                .next()
            {
                Some(v) => v,
                //TODO:添加软件包缺失时的处理程序
                //1. 请求用户判断链接是否正确，若正确，则重新下载
                None => panic!("Not found package {:?}", i),
            };

            //TODO:同上，最终形成一个可以生成迭代器的函数，提供包类型（cross，base）等然后返回包和
            //脚本路径的迭代器
            let script_path =
                match glob(&(vars::BASE_CONFIG.cross_compile_script_path.clone() + &i + "*sh"))
                    .unwrap()
                    .filter_map(Result::ok)
                    .next()
                {
                    Some(v) => v,
                    //TODO:添加软件包缺失时的处理程序
                    //1. 请求用户判断链接是否正确，若正确，则重新下载
                    None => panic!("Not found package {:?}", i),
                };

            //            let script_path: PathBuf = [
            //                &vars::BASE_CONFIG.cross_compile_script_path,
            //                &(i.clone() + ".sh"),
            //            ]
            //            .iter()
            //            .collect();

            println!("{:?} : {:?} : {:?}", &i, &package_path, &script_path);

            println!("{:?}", &vars::BASE_CONFIG.decompress_script);
            let output = Command::new("tar")
                .arg("xvf")
                .arg(package_path)
                .arg("-C")
                .arg("sources")
                .output()
                .expect("error");
            let out = String::from_utf8(output.stdout).unwrap();
            println!("{}", out);

            let target_path = match glob(&("sources/".to_owned() + &i + "*/"))
                .unwrap()
                .filter_map(Result::ok)
                .next()
            {
                Some(v) => v,
                //TODO:添加软件包缺失时的处理程序
                //1. 请求用户判断链接是否正确，若正确，则重新下载
                None => panic!("Not found package {:?}", i),
            };

            let target_path: PathBuf = [target_path, (i.to_owned() + ".sh").into()]
                .iter()
                .collect();
            println!("{:?}", target_path);
            //let target_script_path: PathBuf = ["./", &i].iter().collect();
            fs::copy(script_path, &target_path)?;

            exec_script(target_path);
        }

        Ok(())
    }

    fn check_data(&self, package_name: String) {
        //检测命令的状态就可以
    }

    fn delete_package(&self, package_path: String) -> std::io::Result<()> {
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
        let lfs_env = match self.check_system_env() {
            Ok(v) => v,
            Err(e) => {
                println!("LFS_ENV ERROR : {}", e);
                //TODO:使用错误常量来定义错误
                std::process::exit(1);
            }
        };

        println!("{}", lfs_env);
        //for entry in glob("binutils*").expect("not fount") {
        //    match entry {
        //        Ok(path) => println!("{:?}", path.display()),
        //        Err(e) => println!("{:?}", e),
        //    }
        //}
        self.install_packages(lfs_env).unwrap();

        Retval::new(())
    }
}

struct EnterChroot {}
impl EnterChroot {
    fn prepare_chroot(&self) {
        //脚本来实现
    }
    fn enter_chroot(&self) -> std::io::Result<()> {
        chroot("/mnt/lfs")?;
        std::env::set_current_dir("/")?;
        Ok(())
    }
}
impl TaskTrait for EnterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.prepare_chroot();
        self.enter_chroot();

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
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
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
