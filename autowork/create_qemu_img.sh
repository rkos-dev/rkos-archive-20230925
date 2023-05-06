#!/bin/bash
#

# install qemu-img and nbd
#
sudo pacman -S qemu-img nbd

# enable nbd
#

qemu-img create -f qcow2 vda.qcow2 30G
sudo modprobe nbd max_part=16

qemu-nbd -c /dev/nbd0 vda.qcow2


sudo parted /dev/nbd0 mklabel msdos
sudo parted /dev/nbd0 mkpart primary fat32 0% 200M
sudo parted /dev/nbd0 mkpart primary ext4 200M 100%

sudo mkfs.vfat /dev/nbd0p1
sudo mkfs.ext4 /dev/nbd0p2

mkdir /mnt/lfs
sudo mount /dev/nbd0p2 /mnt/lfs

mkdir /mnt/lfs/boot
sudo mount /dev/nbd0p1 /mnt/lfs/boot

