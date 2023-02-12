extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use std::process::Command;

struct MakingLfsBootable {}
impl TaskTrait for MakingLfsBootable {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //创建etc/fstab
        //设定linux内核
        //挂载boot
        //编译安装linux内核
        //安装grub引导
        //创建引导文件
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
