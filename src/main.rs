extern crate dagrs;

use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use std::env;

mod build_temp_toolchain;
mod prepare_host_sys;
mod utils;
mod vars;

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

fn main() {
    init_logger(None);
    let t1 = TaskWrapper::new(prepare_host_sys::PreparingSoftware {}, "Task 1");
    //let mut t2 = TaskWrapper::new(prepare_host_sys::PreparingDisk {}, "Task 2");
    //let mut t2 = TaskWrapper::new(prepare_host_sys::PreparingNewFileSystem {}, "task 2");
    let mut dagrs = DagEngine::new();

    //t2.exec_after(&[&t1]);
    //t2.input_from(&[&t1]);

    //dagrs.add_tasks(vec![t1, t2]);
    dagrs.add_tasks(vec![t1]);
    assert!(dagrs.run().unwrap());
    let current_dir = env::current_dir().unwrap();
    println!("{:?}", current_dir);
}

#[cfg(test)]
mod tests {
    use crate::utils;
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

        //TODO:处理下载的逻辑，要验证交叉编译的包都在下载列表中
    }

    #[test]
    fn static_vars_test() {
        let _base_config = &vars::BASE_CONFIG;
        let _base_packages = &vars::BASE_PACKAGES;
        let _cross_packages = &vars::CROSS_COMPILE_PACKAGES;
        let _host_packages = &vars::HOST_PACKAGES;
    }
}
