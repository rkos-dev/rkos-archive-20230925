./configure --prefix=/usr        \
            --disable-static     \
            --enable-thread-safe \
            --docdir=/usr/share/doc/mpfr-4.1.0
make && make html
#TODO:这部分不能跳过，但还是需要调整
make check
make && make install-html
