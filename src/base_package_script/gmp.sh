cp -v configfsf.guess config.guess
cp -v configfsf.sub   config.sub
#TODO:默认设置为通用处理器优化库
#
./configure --prefix=/usr    \
            --enable-cxx     \
            --disable-static \
            --docdir=/usr/share/doc/gmp-6.2.1
make && make html
#TODO:检测测试结果
awk '/# PASS:/{total+=$3} ; END{print total}' gmp-check-log
make install && make install-html

