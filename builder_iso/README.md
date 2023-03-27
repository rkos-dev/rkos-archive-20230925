
# 宿主机环境构建指南

* 安装构建工具

```bash
sudo pacman -S archiso
```

* 修改配置文件

  * 如果计划使用kvm虚拟机用作宿主机，则无需修改
  * 如果计划使用virtualbox一类的虚拟机用做宿主机，则修改user_configureation.json中的"harddrives"键的值为/dev/sda、/dev/sdb、/dev/sdc

* 创建构建过程的配置文件目录

```bash
mkdir -p ./archiso
cd ./archiso
cp -r /usr/share/archiso/configs/releng/* ./
```

* 将配置文件覆盖到配置文件目录中

```bash
cd ./archiso
cp path/to/builder_iso/* ./
```

* 构建宿主机镜像

```bash
cd ./archiso
mkarchiso -v -w work/ -o out/ ./
```

* 使用kvm运行构建好的镜像以安装宿主机系统、

  * 需要确认的配置是，务必配置三块虚拟磁盘，第一块磁盘容量20GB，用于宿主机系统安装，第二块磁盘30GB，用作目标磁盘，第三块磁盘根据实际情况选择，要求第三块磁盘容量+内存容量大于35GB
  * 宿主机镜像启动后大约5分钟内会自动运行安装程序，安装程序启动后，需要手动选择install选项，不需要设置任何选项
  * 如果没有自动运行或者运行错误则手动执行指令 ```archinstall --config user_configuration.json --creds user_credentials.json --disk_layouts user_disk_layout.json```
  * 安装完毕后会提示是否进入chroot环境，这里选择no然后reboot即可
  * 宿主机环境安装好后需要手动挂载交换分区

* 自动化脚本（Comming soon）

## 参考文献

[https://wiki.archlinux.org/title/archiso](https://wiki.archlinux.org/title/archiso)

[https://github.com/archlinux/archinstall](https://github.com/archlinux/archinstall)

[https://github.com/archlinux/archinstall/wiki/Building-and-Testing](https://github.com/archlinux/archinstall/wiki/Building-and-Testing)
