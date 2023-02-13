mkdir -v build && pushd build 

../libstdc++-v3/configure           \
    --host=$LFS_TGT                 \
    --build=$(../config.guess)      \
    --prefix=/usr                   \
    --disable-multilib              \
    --disable-nls                   \
    --disable-libstdcxx-pch         \
    --with-gxx-include-dir=/tools/$LFS_TGT/include/c++/12.2.0


if [ "$?" -eq 1 ];
then
    exit $?
fi

make && make DESTDIR=$LFS install

popd

rm -v $LFS/usr/lib/lib{stdc++,stdc++fs,supc++}.la
