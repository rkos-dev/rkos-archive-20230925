extern crate dagrs;

use crate::utils;
use crate::utils::ProgramEndingFlag;
use crate::vars;
use dagrs::{init_logger, DagEngine, DagError, EnvVar, Inputval, Retval, TaskTrait, TaskWrapper};
use log::{error, info};
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn exec_config_script(script_path: PathBuf) -> bool {
    let stdout_file = match File::create("/root/config.log") {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let stderr_file = match stdout_file.try_clone() {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let stdout = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);

    let output = match Command::new("/bin/bash")
        .env_clear()
        .env("PATH", "/usr/bin:/usr/sbin")
        .env("HOME", "/root")
        .env("TERM", "$TERM")
        .arg("-e")
        .arg(script_path)
        .stdout(stdout)
        .stderr(stderr)
        .status()
    {
        Ok(v) => v,
        Err(_e) => return false,
    };
    output.success()
}

pub struct Fstab {}
impl utils::ProgramEndingFlag for Fstab {}
impl TaskTrait for Fstab {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_fstab.sh",
        ]
        .iter()
        .collect();

        let stdout_file = match File::create("/root/config.log") {
            Ok(v) => v,
            Err(_e) => panic!(),
        };

        let stderr_file = match stdout_file.try_clone() {
            Ok(v) => v,
            Err(_e) => panic!(),
        };

        let stdout = Stdio::from(stdout_file);
        let stderr = Stdio::from(stderr_file);

        let output = match Command::new("/bin/bash")
            .env_clear()
            .env("PATH", "/usr/bin:/usr/sbin")
            .env("HOME", "/root")
            .env("TERM", "$TERM")
            .arg("-e")
            .arg(script_path)
            .arg(vars::ROOT_UUID.clone())
            .arg(vars::BOOT_UUID.clone())
            .stdout(stdout)
            .stderr(stderr)
            .status()
        {
            Ok(v) => v,
            Err(e) => {
                error!("Exec fstab config script failed {}", e);
                self.try_set_flag(false);
                panic!();
            }
        };

        match output.success() {
            true => (),
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct Inputrc {}
impl utils::ProgramEndingFlag for Inputrc {}
impl TaskTrait for Inputrc {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_inputrc.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct Network {}
impl utils::ProgramEndingFlag for Network {}
impl TaskTrait for Network {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_network.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct Profile {}
impl utils::ProgramEndingFlag for Profile {}
impl TaskTrait for Profile {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_profile.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct RcSite {}
impl utils::ProgramEndingFlag for RcSite {}
impl TaskTrait for RcSite {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_rc_site.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct Shell {}
impl utils::ProgramEndingFlag for Shell {}
impl TaskTrait for Shell {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_shell.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct Sysvinit {}
impl utils::ProgramEndingFlag for Sysvinit {}
impl TaskTrait for Sysvinit {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_sysvinit.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct Time {}
impl utils::ProgramEndingFlag for Time {}
impl TaskTrait for Time {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_time.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => {}
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

//TODO:kernel 部分如果按照单独的安装，还是需要有按照软件安装的流程，所以先放到软件的部分，后续再优
//化
pub struct InstallKernel {}
impl utils::ProgramEndingFlag for InstallKernel {}
impl TaskTrait for InstallKernel {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_kernel.sh",
        ]
        .iter()
        .collect();
        match exec_config_script(script_path) {
            true => (),
            false => self.try_set_flag(false),
        };

        Retval::empty()
    }
}

pub struct ConfigGrub {}
impl utils::ProgramEndingFlag for ConfigGrub {}
impl TaskTrait for ConfigGrub {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        //执行脚本
        let script_path: PathBuf = [
            &vars::BASE_CONFIG.scripts_path.root,
            &vars::BASE_CONFIG.scripts_path.sysconfig,
            "config_grub.sh",
        ]
        .iter()
        .collect();

        let stdout_file = match File::create("/root/grub.log") {
            Ok(v) => v,
            Err(_e) => panic!(),
        };

        let stderr_file = match stdout_file.try_clone() {
            Ok(v) => v,
            Err(_e) => panic!(),
        };

        let stdout = Stdio::from(stdout_file);
        let stderr = Stdio::from(stderr_file);

        let output = match Command::new("/bin/bash")
            .env_clear()
            .env("PATH", "/usr/bin:/usr/sbin")
            .env("HOME", "/root")
            .env("TERM", "$TERM")
            .arg("-e")
            .arg(script_path)
            .arg(vars::BOOT_UUID.clone())
            .arg(vars::ROOT_PARTUUID.clone())
            .stdout(stdout)
            .stderr(stderr)
            .status()
        {
            Ok(v) => v,
            Err(e) => {
                error!("Exec grub install script failed {}", e);
                self.try_set_flag(false);
                panic!();
            }
        };

        match output.success() {
            true => (),
            false => self.try_set_flag(false),
        };
        Retval::empty()
    }
}
