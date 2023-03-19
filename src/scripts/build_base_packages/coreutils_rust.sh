make

make PREFIX=/usr SKIP_UTILS='kill uptime' install

mv -v /usr/bin/chroot /usr/sbin
