curl -o rust.sh --proto '=https' --tlsv1.2 -ssf https://sh.rustup.rs 

chmod +x rust.sh

./rust.sh -y

rustup install 1.62.0

rustup default 1.62.0


make PREFIX=/usr SKIP_UTILS='kill uptime' install

mv -v /usr/bin/chroot /usr/sbin
