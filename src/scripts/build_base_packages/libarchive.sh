./configure --prefix=/usr --disable-static &&
make
make install

sed -i "s/iconv //" /usr/lib/pkgconfig/libarchive.pc
