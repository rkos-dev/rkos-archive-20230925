./configure --prefix=/usr                            \
            --docdir=/usr/share/doc/procps-ng-4.0.0 \
            --disable-static                         \
            --disable-kill
make && make install
