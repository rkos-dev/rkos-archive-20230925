./configure --prefix=/usr   \
            --without-guile \
            --host=$LFS_TGT \
            --build=$(build-aux/config.guess)

if [ "$?" -eq 1 ];
then
    exit $?
fi

make && make DESTDIR=$LFS install
