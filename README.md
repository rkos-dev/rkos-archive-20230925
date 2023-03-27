# rkos
Rust King OS - Linux Distro of Rust Programing Language

# Tutorial

## 宿主机的准备工作

### 注意事项

- 请确认目前使用的是root用户，并且在所有构建流程都应该使用root用户，另外不推荐使用物理机，虽然构建程序理论上并不会破坏宿主机环境，为了安全起见，建议使用虚拟机

- 请确认内存+交换分区总大小在35GB及以上，如果使用了构建程序提供的宿主机环境，请保证第三块磁盘容量足够，并且已经挂载为交换分区

### 使用提供的宿主机环境

- 运行version-check.sh查看输出，判断是否存在软件包缺失等问题

- 构建宿主系统，并使用kvm启动宿主系统

- 挂载目标分区
    ```
    mkdir -pv /mnt/lfs
    mkdir -pv /mnt/lfs/boot
    cp /etc/fstab{,.DISK_origin}
    echo "/dev/vdb1 /mnt/lfs/boot vfat defaults 1 1" >> /etc/fstab
    echo "/dev/vdb2 /mnt/lfs ext4 defaults 1 1" >> /etc/fstab
    mount -a
    systemctl daemon-reload
    ```

- 编译构建工具

- 将配置文件(configs,scripts,config-6.1,umount.sh)以及运行程序(rkos-builder)置于宿主系统目标分区（/mnt/lfs/）下

- 运行rkos-builder --help 查看指令，并按选项流程构建

- 构建完成后在主机上压缩目标分区qcow2镜像，镜像目录在宿主机kvm镜像存放的位置处

    ```
    TMPDIR=/home/tmp/path virt-sparsity --compress xxx.qcow2 xxx_compress.qcow2
    ```

### 使用自己的Linux

- 安装宿主机必需的软件
    ```
    Bash >= 3.2 (/bin/sh 必须是到 bash 的符号链接或硬连接)
    Binutils >= 2.13.1
    Bison >= 2.7 (/usr/bin/yacc 必须是到 bison 的链接，或者是一个执行 bison 的小脚本)
    Coreutils >= 6.9
    Diffutils >= 2.8.1
    Findutils >= 4.2.31
    Gawk >= 4.0.1 (/usr/bin/awk 必须是到 gawk 的链接)
    GCC >= 4.8，包括 C++ 编译器 g++ ，C 和 C++ 标准库 (包括头文件) 也必须可用，这样 C++ 编译器才能构建宿主环境的程序
    Grep >= 2.5.1a
    Gzip >= 1.3.12
    Linux Kernel >= 3.2
    M4 >= 1.4.10
    Make >= 4.0
    Patch >= 2.5.4
    Perl >= 5.8.8
    Python >= 3.4
    Sed >= 4.1.5
    Tar >= 1.22
    Texinfo >= 4.7
    Xz >= 5.0.0
    ```

- 运行version-check.sh查看输出，判断是否存在软件包缺失等问题

- 创建一个大小为30G的qcow2镜像

    ```
    qemu-img create -f qcow2 vda.qcow2 30G
    ```

- 设置分区并挂载

    ```
    sudo modprobe nbd max_part=16
    qemu-nbd -c /dev/nbdx vda.qocw2
    sudo parted /dev/nbdx mklabel msdos
    sudo parted /dev/nbdx mkpart primary fat32 0% 200M
    sudo parted /dev/nbdx mkpart primary ext4 200M 100%
    sudo mkfs.vfat /dev/nbdxp1
    sudo mkfs.ext4 /dev/nbdxp2
    sudo mount /dev/nbdxp1 /mnt/lfs
    mkdir /mnt/lfs/boot
    sudo mount /dev/nbdxp2 /mnt/lfs/boot

    ```
- 调整配置文件（根据需求调整）

    - 文件路径：configs/base_configs.json

    - 将"path":{"install_path":"设置为/mnt/lfs/或者自己挂载镜像的其他路径，但是要确保在/mnt目录下"}

    - 将"envs":["name":"LFS","value":"设置为上述相同的路径"]

    注意路径末尾需要有'/'符号

- 编译构建工具

- 将配置文件(configs,scripts,config-6.1,umount.sh)以及运行程序(rkos-builder)置于宿主系统目标分区（/mnt/lfs/）下

- 运行rkos-builder --help 查看指令，并按选项流程构建

- 构建完成后在主机上压缩目标分区qcow2镜像，镜像目录在宿主机kvm镜像存放的位置处

    ```
    TMPDIR=/home/tmp/path virt-sparsity --compress xxx.qcow2 xxx_compress.qcow2
    ```

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
# 配置宿主机环境
rkos-builder host-config start 
```

- 构建时可以按照<BUILD_OPTION>中的选项，从host-config开始手动构建每一步，在build-temp-toolchains之后每一步构建完成后需要手动运行umount.sh然后再开始下一步

- build-base-packages由于需要构建clang，根据机器性能不同可能会需要2小时以上的时间，并且需要宿主机挂载有20GB的交换分区

- 可以直接使用build选项来构建所有流程，暂时还不稳定，可能会出现问题，构建的日志会记录在目标分区的root/prepare.log、root/config.log 和root/log.log文件中，分别记录宿主机环境准备过程中的日志和配置中的日志以及构建过程中的日志

# Issue

- e2fsck 版本太旧，内核启动时会产生一个错误，不影响启动，即将修复

- 宿主机网络不通会导致配置和安装失败

- rust包（rust,coreutils,kernel）的构建和安装过程的日志输出不会被记录，即将修复

- 可能出现由于网络不稳定导致rust-src下载失败，重新开始对应option即可

- 软件包安装验证功能缺少，可能出现软件包漏安装，目前测试可能会出现1-2个包漏安装

- 如果安装过程中出现问题，修复问题后，在对应的配置文件中暂时删除已经安装的软件包即可继续构建

    ```
    cp configs/[base_packages.json|temp_toolchains.json|rust_support_packages.json]{,.bak}

    #然后找到对应安装失败的软件包，将之前安装好的全部删掉，解决安装失败的问题后，可以继续运行构建流程
    ```
