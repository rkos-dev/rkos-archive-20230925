./configure --prefix=/usr   \
            --host=$LFS_TGT


make && make DESTDIR=$LFS install
