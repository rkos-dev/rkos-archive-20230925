#!/bin/bash
#

if ! type mkarchiso > /dev/null 2>&1; then
    echo "archiso package not install"
    sudo pacman -S archiso
fi

if [ ! -d /usr/share/archiso/releng ];then
    echo "archiso package not install"
    sudo pacman -S archiso
fi

git clone https://github.com/open-rust-initiative/rkos.git

mkdir target_archiso && cd target_archiso

cp -r /usr/share/archiso/configs/releng/* ./

cp -r ../rkos/builder_iso/airootfs ../rkos/builder_iso/package.x86_64 ./

sudo mkarchiso -v -w work/ -o out/ ./

sudo rm -rf work/
