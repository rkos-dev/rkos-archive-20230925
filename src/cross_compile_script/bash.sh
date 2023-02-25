./configure --prefix=/usr                   \
            --build=$(support/config.guess) \
            --host=$LFS_TGT                 \
            --without-bash-malloc


make && make DESTDIR=$LFS install

rm $LFS/bin/sh
ln -sv bash $LFS/bin/sh
