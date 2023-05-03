cd $LFS
mkdir -v $LFS/sources
chmod -v a+wt $LFS/sources

mkdir -pv $LFS/{etc,var} $LFS/usr/{bin,lib,sbin} $LFS/root

for i in bin lib sbin; do
  ln -sv usr/$i $LFS/$i
done

case $(uname -m) in
  x86_64) mkdir -pv $LFS/lib64 ;;
esac

mkdir -pv $LFS/tools

ln -sv $LFS/tools /

