extern crate dagrs;

use crate::vars;
use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use flate2::read::GzDecoder;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use tar::Archive;

fn exec_script(script_path: PathBuf) {
    let output = Command::new("/bin/bash")
        .arg(script_path)
        .output()
        .expect("error");
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);
}

struct CompilingCrossToolChain {}
impl CompilingCrossToolChain {
    fn check_system_env() {}
    fn install_packages() -> Result<(), std::io::Error> {
        let cross_compile_package = &vars::CROSS_COMPILE_PACKAGES.cross_compile_toolchains;
        for i in cross_compile_package {
            let path = "sources/".to_string() + &i;
            let tar_gz = File::open(path)?;
            let mut archive = Archive::new(tar_gz);
            archive.unpack("./".to_string() + &i)?;

            env::set_current_dir(&i).unwrap();

            let mut current_path = vars::ROOT_DIR.clone();
            current_path.push("cross_compile_scripts");
            current_path.push(i.clone() + ".sh");

            exec_script(current_path);
        }

        Ok(())
    }
    fn check_output_data() {}
    fn delete_package() {}
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
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}

struct EnterChroot {}
impl TaskTrait for EnterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //修改临时环境目录的所有者
        //挂载内核文件系统
        //进入chroot环境
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
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
