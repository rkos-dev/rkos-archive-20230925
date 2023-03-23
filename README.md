# rkos
Rust King OS - Linux Distro of Rust Programing Language

# Tutorial

- 构建宿主系统，并使用kvm启动宿主系统

- 编译后将配置文件(configs,scripts,config-6.1,umount.sh)以及运行程序(rkos-builder)置于宿主系统目标分区下

- 运行lfs_pro --help 分阶段构建

- 构建完成后在主机上压缩目标分区qcow2镜像,可用于kvm启动

# Command


```
rkos-builder --help

Usage: rkos-builder [OPTIONS] <BUILD_OPTION> <OPERATE> [PACKAGE_NAME]

Arguments:

    <BUILD_OPTION>  possible values: 
                    build,
                    host-config,
                    package-download,
                    build-temp-toolchains,
                    build-base-packages,
                    config-target-system,
                    build-rust-support-package-and-kernel,
                    install-grub,
                    clean-up        #Comming soon

    <OPERATE>       possible values:
                    start,
                    reset           #Comming soon

    <PACKAGE_NAME>  default:NULL    #Comming soon

Options:
    -c, --config <DIR>              #Comming soon
    -d, --debug...                  #Comming soon
    -h, --help      Print help
    -V, --version   PrintVersion
```
## Example：

```
# 配置宿主机挂载
rkos-builder host-config start 
```

- 构建时可以按照<BUILD_OPTION>中的选项，从host-config开始手动构建每一步，在build-temp-toolchains之后每一步构建完成后需要手动运行umount.sh然后再开始下一步

- build-base-packages由于需要构建clang，根据机器性能不同可能会需要2小时以上的时间，并且需要宿主机挂载有20GB的交换分区

- 可以直接使用build选项来构建所有流程，暂时还不稳定，可能会出现问题，构建的日志会记录在目标分区的root/config.log 和root/log.log文件中，分别记录配置中的日志和构建过程中的日志

# Issue

- e2fsck 版本太旧，内核启动时会产生一个错误，不影响启动，很快就会修复

- kvm宿主机网络不通会导致配置和安装失败

- rust 包的日志输出没有被记录

- 可能出现由于网络不稳定导致rust-src下载失败

- 需要一个软件包是否安装成功的检测功能
