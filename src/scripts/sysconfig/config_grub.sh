grub-install /dev/vdb

#cat > /boot/grub/grub.cfg << "EOF"
echo "# Begin /boot/grub/grub.cfg
set default=0
set timeout=5

insmod ext2

search --set=root --fs-uuid $1

menuentry \"GNU/Linux, Linux 6.1-rkos-0.0.1\" {
        linux   /vmlinuz-6.1-rkos-0.0.1 root=PARTUUID=$2 ro
}">/boot/grub/grub.cfg


