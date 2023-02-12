extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use std::process::Command;

struct SystemConfiguration {}
impl TaskTrait for SystemConfiguration {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //安装bootstrap
        //处理udev
        //管理设备
        //配置网络
        //配置systemvinit
        //配置bash shell
        //配置inputrc
        //创建etc/shells
        //
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
