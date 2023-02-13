mkdir -v build && pushd build
../configure --prefix=$LFS/tools \
             --with-sysroot=$LFS \
             --target=$LFS_TGT   \
             --disable-nls       \
             --enable-gprofng=no \
             --disable-werror

if [ "$?" -eq 1 ];
then
    exit $?
fi

make && make install

popd
