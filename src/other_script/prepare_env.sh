#!/bin/bash

#默认加载LFS系统变量
cp /root/.bash_profile{,.LFS_origin}
if [ $LFS ];then 
    echo "LFS=$LFS" 
else
    echo "export LFS=/mnt/lfs" >> /root/.bash_profile
fi

#系统默认挂载硬盘，TODO:这部分硬盘设备名还得实际调整，如果可以动态是最好的
mkdir -pv $LFS
mkdir -pv $LFS/boot
cp /etc/fstab{,.DISK_origin}
echo "/dev/vdb1 /mnt/lfs/boot vfat defaults 1 1" >> /etc/fstab
echo "/dev/vdb2 /mnt/lfs ext4 defaults 1 1" >> /etc/fstab
mount -a
systemctl daemon-reload

#默认设定目标系统的环境变量，TODO:后续收尾需要替换名字
cp /root/.bash_profile{,.TGT_origin}
if [ $LFS_TGT ]; then
    echo "LFS_TGT=$LFS_TGT"
else
    echo "export LFS_TGT=$(uname -m)-rkos-linux-gnu" >> /root/.bash_profile
fi

#设定LC_ALL，PATH，CONFIG_FILE的环境变量
cp /root/.bash_profile{,.PATH_origin}
if [ $LC_ALL ];then
    echo "LC_ALL=$LC_ALL"
else
    echo "export LC_ALL=POSIX" >> /root/.bash_profile
fi

PATH=/usr/bin
if [ ! -L /bin ]; then PATH=/bin:$PATH; fi
PATH=/tools/bin:$PATH
echo "export PATH=$PATH" >> /root/.bash_profile

if [ $CONFIG_FILE ];then
    echo "CONFIG_FILE=$CONFIG_FILE"
else
    CONFIG_FILE=$LFS/usr/share/config.site
fi
echo "export CONFIG_FILE=$CONFIG_FILE" >> /root/.bash_profile

CPUS=$(cat /proc/cpuinfo | grep "processor" | wc -l)
if [ $MAKEFLAGS ];then
    echo "MAKEFLAGS=$MAKEFLAGS"
else
    echo "export MAKEFLAGS='-j$CPUS'" >> /root/.bash_profile
fi

echo "export FORCE_UNSAFE_CONFIGURE=1" >> /root/.bash_profile

echo "export NINJAJOBS=$CPUS" >> /root/.bash_profile


source /root/.bash_profile

