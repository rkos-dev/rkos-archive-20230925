use clap::Parser;
use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::env;

mod build_lfs_sys;
mod build_temp_toolchain;
mod prepare_host_sys;
mod utils;
mod vars;
use cmd_lib::*;
use std::path::PathBuf;

struct OutputLfsImg {}
impl TaskTrait for OutputLfsImg {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        Retval::new(())
        // TODO: 导出img文件
    }
}

struct CreateVmBack {}

impl TaskTrait for CreateVmBack {
    fn run(&self, mut input: Inputval, _env: EnvVar) -> Retval {
        // TODO:调用kvm api每个阶段都创建一个备份
        Retval::new(())
    }
}

//TODO:增加流程控制
struct DagNodes {
    prepare_env: bool,
    prepare_disk: bool,
    check_env: bool,
    prepare_software: HashMap<String, bool>,
    compile_cross_toolchain_check: bool,
    compile_cross_toolchain: HashMap<String, bool>,
    prepare_chroot_chmod: bool,
    prepare_virt_sys: bool,
    enter_chroot: bool,
    create_path: bool,
    compile_temp_packages: bool,
    clean_up: bool,
    build_basic_system: bool,
    config_sys: bool,
    output: bool,
}

fn main() {
    init_logger(None);
    let prepare = TaskWrapper::new(prepare_host_sys::Prepare {}, "Prepare");
    let compile_temp_packages = TaskWrapper::new(
        build_temp_toolchain::CompilingCrossToolChain {},
        "Compile temp packages",
    );
    let build_base_system = TaskWrapper::new(
        build_lfs_sys::InstallBasicSystemSoftware {},
        "Build base system",
    );
    //let t1 = TaskWrapper::new(build_temp_toolchain::CompilingCrossToolChain {}, "Task 1");
    let mut dag_nodes = vec![prepare, compile_temp_packages, build_base_system];

    ////let mut t2 = TaskWrapper::new(prepare_host_sys::PreparingDisk {}, "Task 2");
    ////let mut t2 = TaskWrapper::new(prepare_host_sys::PreparingNewFileSystem {}, "task 2");

    let mut dagrs = DagEngine::new();
    ////TODO:python-doc需要调整包名，libstdc++需要调整包名，tcl-doc需要调整包名，zlib包会随着版本更新
    ////而链接失效，libstdc++只需要下载gcc之后copy一份成为libstdc++就可以
    ////python tcl 解决了 明天需要确认

    ////t2.exec_after(&[&t1]);
    ////t2.input_from(&[&t1]);

    dagrs.add_tasks(dag_nodes);
    assert!(dagrs.run().unwrap());

    let current_dir = env::current_dir().unwrap();
    info!("{:?}", current_dir);
}

#[cfg(test)]
mod tests {
    use crate::vars;

    #[test]
    fn package_test() {
        let base_path =
            "/home/jxy/workspace/all_record/openEuler_pro_1/pro/src/lfs_pro/src/".to_string();
        let base_packages = vars::parse_json::<vars::BasePackages>(
            &(base_path.clone() + "configs/base_packages.json"),
        );
        let cross_packages = vars::parse_json::<vars::CrossCompilePackages>(
            &(base_path.clone() + "configs/cross_compile_packages.json"),
        );
        let base_config =
            vars::parse_json::<vars::BaseConfig>(&(base_path.clone() + "configs/base_config.json"));
        assert!(base_packages.is_ok());
        assert!(cross_packages.is_ok());
        assert!(base_config.is_ok());
    }

    #[test]
    fn static_vars_test() {
        let _base_config = &vars::BASE_CONFIG;
        let _base_packages = &vars::BASE_PACKAGES;
        let _cross_packages = &vars::CROSS_COMPILE_PACKAGES;
        let _host_packages = &vars::HOST_PACKAGES;
    }
}
