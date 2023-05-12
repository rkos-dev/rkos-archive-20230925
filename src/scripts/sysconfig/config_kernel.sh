
rustup component add rust-src

cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen

rustup component add rustfmt

rustup component add clippy


make CC=clang  rustavailable

cp /config-6.1 ./.config

#make CC=clang allnoconfig /config-6.1

make CC=clang oldconfig

make CC=clang

make CC=clang modules_install

cp -iv arch/x86_64/boot/bzImage /boot/vmlinuz-6.1-rkos-0.0.1

cp -iv System.map /boot/System.map-6.1

cp -iv .config /boot/config-6.1


install -d /usr/share/doc/linux-6.1
cp -r Documentation/* /usr/share/doc/linux-6.1
