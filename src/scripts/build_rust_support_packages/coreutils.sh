curl --proto '=https' --tlsv1.2 -ssf https://sh.rustup.rs | sh

rustup install 1.62.0

make

make PREFIX=/usr SKIP_UTILS='kill uptime' install

mv -v /usr/bin/chroot /usr/sbin
