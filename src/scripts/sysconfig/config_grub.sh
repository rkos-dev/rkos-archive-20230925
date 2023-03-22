grub-install /dev/vdb

cat > /boot/grub/grub.cfg << "EOF"
# Begin /boot/grub/grub.cfg
set default=0
set timeout=5

insmod ext2

search --set=root --fs-uuid $1

menuentry "GNU/Linux, Linux 6.1-rkos-0.0.1" {
        linux   /boot/vmlinuz-6.1-rkos-0.0.1 root=PARTUUID=$2 ro
}
EOF


