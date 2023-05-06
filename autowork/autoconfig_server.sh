#!/bin/bash
# install dep packages
sudo pacman -S wget git rust clang parted pkg-config

# clone source code
git clone https://github.com/xyyy1420/rkos.git

# build rkos-builder
#
cd rkos
cargo build --release

cd ../


