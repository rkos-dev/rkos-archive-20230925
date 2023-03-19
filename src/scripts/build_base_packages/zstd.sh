patch -Np1 -i ../base_patches/zstd-1.5.2-upstream_fixes-1.patch

make prefix=/usr && make prefix=/usr install

rm -v /usr/lib/libzstd.a
