use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::chroot;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;

use glob::glob;

fn exec_build_script(script_path: PathBuf, dir: PathBuf) -> bool {
    let abs_path = fs::canonicalize(dir.as_path()).unwrap();
    let filename = match script_path.to_str() {
        Some(v) => v,
        None => panic!("cannot turn to str"),
    };
    let output = Command::new("/bin/bash")
        .current_dir(abs_path)
        .arg(filename)
        .status()
        .expect("error");
    output.success()
}

pub fn target_path_exists(path: &str) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new(&path);
    match target_path.exists() {
        true => Ok(()),
        false => Err("File/Dir not exists".into()),
    }
}

pub fn build_packages(package_name: String) {
    match target_path_exists(&package_name) {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn lfs_chroot(path: &str) -> std::io::Result<()> {
    chroot(path)?;
    std::env::set_current_dir("/")?;
    Ok(())
}

pub fn install_package(package_name: String, script_path: String) {
    //软件包相对工作目录的路径
    let package_path = match glob(&("sources/".to_owned() + &package_name + "*"))
        .unwrap()
        .filter_map(Result::ok)
        .next()
    {
        Some(v) => v,
        //TODO:添加软件包缺失时的处理程序
        //1. 请求用户判断链接是否正确，若正确，则重新下载
        None => panic!("Not found package {:?}", package_name),
    };

    //预先准备的脚本文件路径
    let script_full_path = match glob(&(script_path + &package_name + "*sh"))
        .unwrap()
        .filter_map(Result::ok)
        .next()
    {
        Some(v) => v,
        //TODO:添加软件包缺失时的处理程序
        //1. 请求用户判断链接是否正确，若正确，则重新下载
        None => panic!("Not found script {:?}", package_name),
    };

    println!(
        "{:?} : {:?} : {:?}",
        &package_name, &package_path, &script_full_path
    );

    let output = Command::new("tar")
        .arg("xvf")
        .arg(package_path)
        .arg("-C")
        .arg("sources")
        .output()
        .expect("error");
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);

    //解压好的程序包路径
    let target_path = match glob(&("sources/".to_owned() + &package_name + "*/"))
        .unwrap()
        .filter_map(Result::ok)
        .next()
    {
        Some(v) => v,
        //TODO:添加软件包缺失时的处理程序
        //1. 请求用户判断链接是否正确，若正确，则重新下载
        None => panic!("Not found targetpath {:?}", package_name),
    };

    //最终脚本文件在程序包中的路径
    let script_target_path: PathBuf = [
        target_path.clone(),
        (package_name.to_owned() + ".sh").into(),
    ]
    .iter()
    .collect();

    println!("{:?} : {:?}", &target_path, script_target_path);
    fs::copy(script_full_path, &script_target_path).unwrap();

    //脚本文件名字
    let script_name = match script_target_path.file_name() {
        Some(v) => v,
        None => panic!("err"),
    };

    let status = exec_build_script(script_name.into(), target_path.clone());
    fs::remove_dir_all(target_path).unwrap();
    assert!(status);
}
