use clap::Parser;
use dagrs::{init_logger, DagEngine, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::env;
use std::fs;

mod build_lfs_sys;
mod build_temp_toolchain;
mod prepare_host_sys;
mod utils;
mod vars;
use cmd_lib::*;
use std::path::Path;
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

#[warn(dead_code)]
fn init() {
    let stop_flag_path = Path::new("./stop");
    if stop_flag_path.exists() {
        if let Err(e) = fs::remove_file(stop_flag_path) {
            panic!("Cannot init stop flag {}", e);
        }
    }
}

fn main() {
    init();
    init_logger(None);
    let cli = vars::Cli::parse();

    //检测是否在必要的工作目录下
    let current_dir = env::current_dir().unwrap();
    if current_dir.parent() != Some(Path::new("/mnt")) {
        error!(
            "Cannot run at path {:?} , you must take the program in /mnt/[TEMP_SYSTEM]",
            current_dir
        );
        return;
    }

    //检查环境变量
    let check_env = TaskWrapper::new(prepare_host_sys::CheckEnv {}, "Check host env");

    //准备配置目标文件目录
    let mut prepare_dirs = TaskWrapper::new(prepare_host_sys::PreparingDirs {}, "PREPARE DIRS");

    //准备下载软件
    let mut prepare_software =
        TaskWrapper::new(prepare_host_sys::PreparingSoftware {}, "Prepare software");

    //安装临时工具链
    let mut compile_toolchains = TaskWrapper::new(
        build_temp_toolchain::CompilingCrossToolChain {},
        "Compile Toolchains",
    );

    //进入chroot环境
    let mut enter_chroot = TaskWrapper::new(utils::EnterChroot {}, "Enter Chroot");

    //配置进入chroot环境后的部分
    let mut after_chroot =
        TaskWrapper::new(build_temp_toolchain::AfterChroot {}, "After chroot config");

    //安装chroot后的工具
    let mut install_other_packages = TaskWrapper::new(
        build_temp_toolchain::AfterChrootInstall {},
        "Install other packages",
    );

    //清理系统
    let mut clean_system = TaskWrapper::new(
        build_temp_toolchain::CleanUpAndSaveTempSystem {},
        "Clean up",
    );

    //安装基础软件包
    let mut install_packages = TaskWrapper::new(
        build_lfs_sys::InstallBasicSystemSoftware {},
        "Install Packages",
    );
    //删除debug的符号
    let mut remove_debug_symbol =
        TaskWrapper::new(build_lfs_sys::RemoveDebugSymbol {}, "Remove debug symbol");
    //清理系统
    let mut clean_up_system = TaskWrapper::new(build_lfs_sys::CleanUpSystem {}, "Clean up system");

    let mut dagrs = DagEngine::new();
    let mut dag_nodes: Vec<TaskWrapper> = Vec::new();

    match &cli.build_option {
        vars::BuildOption::Build => {
            prepare_dirs.exec_after(&[&check_env]);

            prepare_software.exec_after(&[&prepare_dirs]);

            dag_nodes.push(check_env);

            dag_nodes.push(prepare_dirs);

            dag_nodes.push(prepare_software);
        }

        vars::BuildOption::HostConfig => {
            prepare_dirs.exec_after(&[&check_env]);

            dag_nodes.push(check_env);

            dag_nodes.push(prepare_dirs);
        }
        vars::BuildOption::PackageDownload => {
            dag_nodes.push(prepare_software);
        }
        vars::BuildOption::BuildTempToolchains => {
            compile_toolchains.exec_after(&[&check_env]);
            enter_chroot.exec_after(&[&compile_toolchains]);
            after_chroot.exec_after(&[&enter_chroot]);
            install_other_packages.exec_after(&[&after_chroot]);
            clean_system.exec_after(&[&install_other_packages]);

            dag_nodes.push(check_env);
            dag_nodes.push(compile_toolchains);
            dag_nodes.push(enter_chroot);
            dag_nodes.push(after_chroot);
            dag_nodes.push(install_other_packages);
            dag_nodes.push(clean_system);
        }
        vars::BuildOption::BuildBackPackages => {
            dag_nodes.push(install_packages);
        }
        vars::BuildOption::CleanUp => {
            dag_nodes.push(remove_debug_symbol);
        }
        vars::BuildOption::ConfigTargetSystem => {}
    }

    match &cli.operate {
        vars::StartMode::Start => {
            dagrs.add_tasks(dag_nodes);
        }
        vars::StartMode::Reset => {}
    };
    assert!(dagrs.run().unwrap());

    ////TODO:python-doc需要调整包名，libstdc++需要调整包名，tcl-doc需要调整包名，zlib包会随着版本更新
    ////而链接失效，libstdc++只需要下载gcc之后copy一份成为libstdc++就可以
    ////python tcl 解决了 明天需要确认

    ////t2.exec_after(&[&t1]);
    ////t2.input_from(&[&t1]);

    //    dagrs.add_tasks(test_dag_nodes);
    //    assert!(dagrs.run().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::vars;

    #[test]
    fn const_vars_test() {
        let root_dir = &vars::ROOT_DIR;
        let base_config = &vars::BASE_CONFIG;
        println!("{}", base_config.scripts_path.root.clone());
        let all_package = &vars::ALL_PACKAGES;
        println!("{}", all_package.all_packages[0].name.clone());
        let cross_compile_packages = &vars::CROSS_COMPILE_PACKAGES;
        println!(
            "{}",
            cross_compile_packages.cross_compile_packages[0]
                .name
                .clone()
        );
        let base_packages = &vars::BASE_PACKAGES;
        println!("{}", base_packages.base_packages[0].name.clone());
    }
}
