use cmd_lib::*;
use dagrs::{EnvVar, Inputval, Retval, TaskTrait};
use log::{error, info};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;

use glob::glob;
use std::os::unix::fs::chroot;
use std::process::Stdio;

use downloader::Downloader;

use crate::vars::BASE_CONFIG;

pub struct InstallInfo {
    pub dir_name: String,
    pub package_name: String,
    pub script_name: String,
    pub script_path: String,
    pub package_source_path: String,
    pub package_target_path: String,
}

pub fn exec_chroot_script(script_path: PathBuf, dir: PathBuf) -> bool {
    //日志输出文件
    let stdout_file = match File::create("/root/log.log") {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let stderr_file = match stdout_file.try_clone() {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let stdio = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);
    //取绝对路径
    let abs_path = match fs::canonicalize(dir.as_path()) {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let output = match Command::new("/bin/bash")
        .current_dir(abs_path)
        .env_clear()
        .env("PATH", "/usr/bin:/usr/sbin:/root/.cargo/bin")
        .env("HOME", "/root")
        .env("TERM", "$TERM")
        .env("MAKEFLAGS", "-j8")
        .env("NINJAJOBS", "8")
        .arg("-e")
        .arg(script_path)
        .stdout(stdio)
        .stderr(stderr)
        .status()
    {
        Ok(v) => v,
        Err(_e) => return false,
    };

    output.success()
}

fn exec_build_script(script_path: PathBuf, dir: PathBuf) -> bool {
    let stdout_file = match File::create("/root/log.log") {
        Ok(v) => v,
        Err(_e) => return false,
    };
    let stderr_file = match stdout_file.try_clone() {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let stdio = Stdio::from(stdout_file);
    let stderr = Stdio::from(stderr_file);
    let abs_path = match fs::canonicalize(dir.as_path()) {
        Ok(v) => v,
        Err(_e) => return false,
    };

    let output = match Command::new("/bin/bash")
        .current_dir(abs_path)
        .env("MAKEFLAGS", "-j8")
        .arg("-e")
        .arg(script_path)
        .stdout(stdio)
        .stderr(stderr)
        .status()
    {
        Ok(v) => v,
        Err(_e) => return false,
    };
    output.success()
}

//FIXME:看情况是否需要单独的配置步骤
pub struct ConfigChrootEnv {}
impl ProgramEndingFlag for ConfigChrootEnv {}
impl TaskTrait for ConfigChrootEnv {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        Retval::empty()
    }
}

pub struct EnterChroot {}
impl ProgramEndingFlag for EnterChroot {}
impl TaskTrait for EnterChroot {
    fn run(&self, _input: Inputval, _env: EnvVar) -> Retval {
        self.check_flag();
        //        let chmod_script_path = "chroot_scripts/chown.sh";
        let chmod_script_path: PathBuf = [
            &BASE_CONFIG.scripts_path.root,
            &BASE_CONFIG.scripts_path.chroot,
            "target_path_chown.sh",
        ]
        .iter()
        .collect();

        match Command::new("/bin/bash")
            .arg("-e")
            .arg(chmod_script_path)
            .status()
        {
            Ok(v) => match v.success() {
                true => {}
                false => {
                    self.try_set_flag(false);
                }
            },
            Err(e) => {
                error!("Failed chown target path , Err msg：{}", e);
                self.try_set_flag(false);
            }
        };

        let mount_virt_fsystem: PathBuf = [
            &BASE_CONFIG.scripts_path.root,
            &BASE_CONFIG.scripts_path.chroot,
            "mount_vfsys.sh",
        ]
        .iter()
        .collect();
        match Command::new("/bin/bash")
            .arg("-e")
            .arg(mount_virt_fsystem)
            .status()
        {
            Ok(v) => match v.success() {
                true => {}
                false => {
                    self.try_set_flag(false);
                }
            },
            Err(e) => {
                error!("Failed mount virt file system , Err msg：{}", e);
                self.try_set_flag(false);
            }
        };

        match chroot("/mnt/lfs") {
            Ok(_v) => {
                info!("success chroot to /mnt/lfs ");
            }
            Err(e) => {
                error!("chroot to /mnt/lfs failed , Err msg {}", e);
                self.try_set_flag(false);
            }
        }
        match std::env::set_current_dir("/") {
            Ok(_v) => {
                info!("success chroot to /mnt/lfs ");
            }
            Err(e) => {
                error!("Set current dir failed , Err msg {}", e);
                self.try_set_flag(false);
            }
        }

        Retval::empty()
    }
}

pub fn env_status(env: String) -> Result<String, bool> {
    if let Ok(v) = env::var(env) {
        return Ok(v);
    }
    Err(false)
}

pub fn download(target_path: String, url: String) -> Result<bool, Box<dyn Error>> {
    let cmd = format!("wget -P {} {}", target_path, url);
    let output = Command::new("/bin/bash").arg("-c").arg(cmd).status()?;
    Ok(output.success())
}

#[allow(unused)]
pub fn new_downlaod(target_path: String, url: &[&str]) -> Result<bool, Box<dyn Error>> {
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new(&target_path))
        .retries(5)
        .parallel_requests(1)
        .build()?;
    let dl = downloader::Download::new_mirrored(&url);
    let result = downloader.download(&[dl])?;
    for r in result {
        match r {
            Ok(_) => return Ok(true),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(true)
}

pub fn install_package(
    package_info: InstallInfo,
    chroot_flag: bool,
) -> Result<bool, Box<dyn Error>> {
    info!("package source path :{};  package name :{};  script path :{}; script name :{};  package_target_path :{};",&package_info.package_source_path,&package_info.package_name,&package_info.script_path,&package_info.script_name,&package_info.package_target_path);

    //软件包相对工作目录的路径
    let package_path =
        match glob(&(package_info.package_source_path.clone() + &package_info.package_name + "*"))?
            .filter_map(Result::ok)
            .next()
        {
            Some(v) => v,
            None => return Err(format!("Not found package {:?}", package_info.package_name).into()),
        };

    info!(
        "Package name : {:?} ; Package path : {:?} ;",
        &package_info.package_name, &package_path,
    );

    //预先准备的脚本文件路径
    let script_full_path =
        match glob(&(package_info.script_path.clone() + &package_info.script_name + "*sh"))?
            .filter_map(Result::ok)
            .next()
        {
            Some(v) => v,
            None => return Err(format!("Not found script {:?}", package_info.package_name).into()),
        };

    info!(
        "Package name : {:?} ; Package path : {:?} ; Script path : {:?}",
        &package_info.package_name, &package_path, &script_full_path
    );

    let output = Command::new("/usr/bin/tar")
        .arg("xf")
        .arg(package_path)
        .arg("-C")
        .arg(package_info.package_target_path.clone())
        .output()
        .expect("error");
    let out = String::from_utf8(output.stdout).unwrap();

    info!("{}", out);

    //解压好的程序包路径
    let target_path =
        match glob(&(package_info.package_target_path + &package_info.dir_name + "*/"))?
            //    let target_path = match glob(&("sources/".to_owned() + &package_name + "*/"))
            .filter_map(Result::ok)
            .next()
        {
            Some(v) => v,
            None => {
                return Err(format!("Can not get target path {}", package_info.package_name).into())
            } //None => panic!("Not found target path {:?}", package_name),
        };

    //最终脚本文件在程序包中的路径
    let script_target_path: PathBuf = [
        target_path.clone(),
        (package_info.script_name.clone() + ".sh").into(),
    ]
    .iter()
    .collect();

    info!(
        "Script source path : {:?} ; Script target path : {:?} ;",
        target_path, script_target_path
    );
    //    println!("{:?} : {:?}", &target_path, script_target_path);
    fs::copy(script_full_path, &script_target_path)?;

    //脚本文件名字
    let fin_script_name = match script_target_path.file_name() {
        Some(v) => v,
        None => return Err("get filename failed".into()),
    };

    let status = match chroot_flag {
        true => exec_chroot_script(fin_script_name.into(), target_path.clone()),
        false => exec_build_script(fin_script_name.into(), target_path.clone()),
    };
    //let status = exec_build_script(fin_script_name.into(), target_path.clone());

    fs::remove_dir_all(target_path.clone()).unwrap();

    match status {
        true => {
            info!("Package {:?} install success", package_info.package_name);
            Ok(true)
        }
        false => Err(format!("Package {:?} install failed", target_path).into()),
    }
}

pub fn delete_failed_download_pack(target_pack_name: &str, target_path: &str) {
    match glob(&(target_path.to_string() + target_pack_name)) {
        Ok(v) => {
            if let Some(v) = v.filter_map(Result::ok).next() {
                fs::remove_file(v).unwrap()
            }
        }
        Err(_e) => {}
    }
}

pub fn check_download_before(target_pack_name: &str, target_path: &str) -> bool {
    match glob(&(target_path.to_string() + target_pack_name)) {
        Ok(v) => v.filter_map(Result::ok).next().is_some(),
        Err(_e) => false,
    }
}

pub trait ProgramEndingFlag {
    fn check_flag(&self) {
        let target_path = Path::new("stop");
        if target_path.exists() {
            panic!("Program Ending");
        }
    }
    fn try_set_flag(&self, flag: bool) {
        let target_path = Path::new("./stop");
        match flag {
            false => {
                //FIXME:直接unwrap虽然可以，但是不太美观
                File::create(target_path).unwrap();
                panic!("Program Ending");
            }
            true => (),
        }
    }
}
