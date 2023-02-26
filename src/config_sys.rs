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
        //bash
        //dev
        //inputrc
        //linuxconsole
        //NetworkConfig
        //rc site
        //shell config
        //sysv boot
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}

struct ManagerDev {}
impl TaskTrait for ManagerDev {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {}
}

struct NetworkConfig {}
impl TaskTrait for NetworkConfig {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {}
}

struct SysvInit {}
impl TaskTrait for SysvInit {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {}
}

struct BashShellConfig {}
impl TaskTrait for BashShellConfig {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {}
}

struct CreateInputrc {}
impl TaskTrait for CreateInputrc {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {}
}

struct CreateShellConfig {}
impl TaskTrait for CreateShellConfig {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {}
}
