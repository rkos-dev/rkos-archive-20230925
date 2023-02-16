mkdir build
pushd build
  ../configure --disable-bzlib      \
               --disable-libseccomp \
               --disable-xzlib      \
               --disable-zlib
  make
popd

./configure --prefix=/usr --host=$LFS_TGT --build=$(./config.guess)

if [ "$?" -eq 1 ];
then
    exit $?
fi

make FILE_COMPILE=$(pwd)/build/src/file && make DESTDIR=$LFS install
rm -v $LFS/usr/lib/libmagic.la
