./configure --prefix=/usr   \
            --host=$LFS_TGT \
            --build=$(build-aux/config.guess)

if [ "$?" -eq 1 ];
then
    exit $?
fi

make && DESTDIR=$LFS install
