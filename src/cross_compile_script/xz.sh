./configure --prefix=/usr                     \
            --host=$LFS_TGT                   \
            --build=$(build-aux/config.guess) \
            --disable-static                  \
            --docdir=/usr/share/doc/xz-5.2.6

if [ "$?" -eq 1 ];
then
    exit $?
fi

make && make DESTDIR=$LFS install
rm -v $LFS/usr/lib/liblzma.la
