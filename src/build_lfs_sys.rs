extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use std::process::Command;

struct InstallBasicSystemSoftware {}
impl TaskTrait for InstallBasicSystemSoftware {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        //按顺序安装软件包
        //在coreutils之前，安装clangd，llvm，rust
        //删除无用文件，保留调试符号
        //
        let hello_dagrs = String::from("Hello Dagrs!");
        Retval::new(hello_dagrs)
    }
}
