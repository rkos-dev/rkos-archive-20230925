#!/bin/bash

#默认加载LFS系统变量
cp /root/.bash_profile{,.origin}
echo "export LFS=/mnt/lfs" >> /root/.bash_profile
source /root/.bash_profile

#系统默认挂载硬盘，TODO:这部分硬盘设备名还得实际调整，如果可以动态是最好的
mkdir -pv $LFS
cp /etc/fstab{,.origin}
echo "/dev/sdb1 /mnt/lfs ext4 defaults 1 1" >> /etc/fstab
mount -a

#默认设定目标系统的环境变量，TODO:后续收尾需要替换名字
cp /root/.bash_profile{,.origin}
echo "export LFS_TGT=$(uname -m)-linux-gnu" >> /root/.bash_profile
source /root/.bash_profile

#设定LC_ALL，PATH，CONFIG_FILE的环境变量
cp /root/.bash_profile{,.origin}
echo "export LC_ALL=POSIX" >> /root/.bash_profile

PATH=/usr/bin
if [ ! -L /bin ]; then PATH=/bin:$PATH; fi
PATH=$LFS/tools/bin:$PATH
echo "export PATH=$PATH" >> /root/.bash_profile

CONFIG_FILE=$LFS/usr/share/config.site
echo "export CONFIG_FILE=$CONFIG_FILE" >> /root/.bash_profile

CPUS=$(cat /proc/cpuinfo | grep "processor" | wc -l)
echo "export MAKEFLAGS='-j$CPUS'" >> /root/.bash_profile

source /root/.bash_profile

