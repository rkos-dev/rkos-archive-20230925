sed '/Deny everything else/i SECCOMP_ALLOW(__NR_getrandom),' \
    -i src/privsep-linux.c

./configure --prefix=/usr                \
            --sysconfdir=/etc            \
            --libexecdir=/usr/lib/dhcpcd \
            --dbdir=/var/lib/dhcpcd      \
            --runstatedir=/run           \
            --disable-privsep         &&
make
make install
