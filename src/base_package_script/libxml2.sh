
./configure --prefix=/usr           \
            --sysconfdir=/etc       \
            --disable-static        \
            --with-history          \
            PYTHON=/usr/bin/python3 \
            --docdir=/usr/share/doc/libxml2-2.10.3 &&
make
make install
