
uuid1=$1
uuid2=$2

# fstab
#cat > /etc/fstab << "EOF"
echo "# Begin /etc/fstab

# 文件系统     挂载点       类型     选项                转储  检查
#                                                              顺序

UUID=${uuid1}     /            ext4     defaults            1     1
UUID=${uuid2}      /boot        vfat     defaults            1     1
proc           /proc        proc     nosuid,noexec,nodev 0     0
sysfs          /sys         sysfs    nosuid,noexec,nodev 0     0
devpts         /dev/pts     devpts   gid=5,mode=620      0     0
tmpfs          /run         tmpfs    defaults            0     0
devtmpfs       /dev         devtmpfs mode=0755,nosuid    0     0

# End /etc/fstab">/etc/fstab
