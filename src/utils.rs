use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs;
use std::path::Path;
use std::result::Result;

use glob::glob;

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

pub fn chroot(path: &str) -> std::io::Result<()> {
    fs::chroot(path)?;
    std::env::set_current_dir("/")?;
    Ok(())
}

//TODO:后续修改为获取绝对路径
//pub fn get_real_path(dir_path: String, package: String) -> Result<(), Box<dyn Error>> {
//    let path = match glob(&(dir_path + &package))
//        .unwrap()
//        .filter_map(Result::ok)
//        .next()
//    {
//        Some(v) => v,
//        None => panic!("not pattern path"),
//    };
//
//    Ok(())
//}
