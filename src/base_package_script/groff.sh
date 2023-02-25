PAGE=A4 ./configure --prefix=/usr
export MAKEFLAGS='j1'
make -j1
make install
