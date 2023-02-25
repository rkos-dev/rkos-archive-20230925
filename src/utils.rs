use log::info;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;

use glob::glob;
use std::process::Stdio;

fn exec_build_script(script_path: PathBuf, dir: PathBuf) -> bool {
    let file = File::create("/root/log.log").unwrap();
    let stdio = Stdio::from(file);
    let abs_path = fs::canonicalize(dir.as_path()).unwrap();
    let filename = match script_path.to_str() {
        Some(v) => v,
        None => panic!("cannot turn to str"),
    };
    let output = Command::new("/bin/bash")
        .current_dir(abs_path)
        .arg("e")
        .arg(filename)
        .stdout(stdio)
        .status()
        .unwrap();
    output.success()
}

pub fn download(target_path: String, url: String) -> Result<bool, Box<dyn Error>> {
    let cmd = format!("wget -P {} {}", target_path, url);
    let output = Command::new("/bin/bash").arg("-c").arg(cmd).status()?;
    Ok(output.success())
}

pub fn target_path_exists(path: &str) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new(&path);
    match target_path.exists() {
        true => Ok(()),
        false => Err("File/Dir not exists".into()),
    }
}

pub fn install_package(
    package_name: String,
    script_path: String,
    script_name: String,
    package_source_path: String,
    package_target_path: String,
) -> Result<bool, Box<dyn Error>> {
    //软件包相对工作目录的路径
    info!("package source path :{};  package name :{};  script path :{}; script name :{};  package_target_path :{};",&package_source_path,&package_name,&script_path,&script_name,&package_target_path);

    let package_full_path = package_source_path.clone() + &package_name + "*";
    info!("{:?}", package_full_path);
    let package_path = match glob(&(package_source_path.clone() + &package_name + "*"))?
        //let package_path = match glob(&("sources/".to_owned() + &package_name + "*"))
        .filter_map(Result::ok)
        .next()
    {
        Some(v) => v,
        //TODO:添加软件包缺失时的处理程序
        //1. 请求用户判断链接是否正确，若正确，则重新下载
        None => return Err(format!("Not found package {:?}", package_name).into()),
    };

    //预先准备的脚本文件路径
    let script_full_path = match glob(&(script_path.clone() + &script_name + "*sh"))?
        .filter_map(Result::ok)
        .next()
    {
        Some(v) => v,
        None => return Err(format!("Not found script {:?}", package_name).into()),
    };

    info!(
        "Package name : {:?} ; Package path : {:?} ; Script path : {:?}",
        &package_name, &package_path, &script_full_path
    );

    //    let cmd = format!("tar -xvf {} -C {}", package_path, package_source_path);
    //    let output = Command::new("/bin/bash")
    //        .arg("-c")
    //        .arg(cmd)
    //        .status()
    //        .expect("error");

    let output = Command::new("/usr/bin/tar")
        .arg("xvf")
        .arg(package_path)
        .arg("-C")
        .arg(package_target_path.clone())
        .output()
        .expect("error");
    let out = String::from_utf8(output.stdout).unwrap();

    //info!("{}", out);

    //解压好的程序包路径
    let target_path = match glob(&(package_target_path + &package_name + "*/"))?
        //    let target_path = match glob(&("sources/".to_owned() + &package_name + "*/"))
        .filter_map(Result::ok)
        .next()
    {
        Some(v) => v,
        //TODO:添加软件包缺失时的处理程序
        //1. 请求用户判断链接是否正确，若正确，则重新下载
        None => return Err(format!("Can not get target path {}", package_name).into()), //None => panic!("Not found target path {:?}", package_name),
    };

    //最终脚本文件在程序包中的路径
    let script_target_path: PathBuf = [target_path.clone(), (script_name.clone() + ".sh").into()]
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
        None => return Err("err".into()),
    };

    let status = exec_build_script(fin_script_name.into(), target_path.clone());

    fs::remove_dir_all(target_path.clone()).unwrap();

    match status {
        true => {
            info!("Package {:?} install success", package_name.clone());
            Ok(true)
        }
        false => Err(format!("Package {:?} install failed", target_path).into()),
    }
}
