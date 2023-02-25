./configure --prefix=/usr \
            --docdir=/usr/share/doc/flex-2.6.4 \
            --disable-static

make && make install

ln -sv flex /usr/bin/lex
