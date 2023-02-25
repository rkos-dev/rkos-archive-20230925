./configure --prefix=/usr    \
            --disable-static \
            --docdir=/usr/share/doc/expat-2.4.8

make && make install
install -v -m644 doc/*.{html,css} /usr/share/doc/expat-2.4.8

