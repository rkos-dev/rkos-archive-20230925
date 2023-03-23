./configure --prefix=/usr --disable-static && make

make install

make -C doc/reference install-data-local
