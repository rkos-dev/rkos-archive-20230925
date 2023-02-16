./configure --prefix=/usr   \
            --host=$LFS_TGT

if [ "$?" -eq 1 ];
then
    exit $?
fi

make && make DESTDIR=$LFS install
