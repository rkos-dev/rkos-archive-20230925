extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use std::collections::HashMap;
use std::process::Command;
use vars;

struct InstallBasicSystemSoftware {}
impl TaskTrait for InstallBasicSystemSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        let mut package_install_list = HashMap::new();
        //按顺序安装软件包
        //在coreutils之前，安装clangd，llvm，rust
        //删除无用文件，保留调试符号
        //
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
