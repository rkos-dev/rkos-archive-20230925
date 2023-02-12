use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs;
use std::path::Path;
use tar::Archive;

pub fn target_path_exists(path: &str) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new(&path);
    match target_path.exists() {
        true => Ok(()),
        false => Err("File/Dir not exists".into()),
    }
}

pub fn unzip(path: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
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
