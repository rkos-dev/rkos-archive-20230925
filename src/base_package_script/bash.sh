./configure --prefix=/usr                      \
            --docdir=/usr/share/doc/bash-5.1.16 \
            --without-bash-malloc              \
            --with-installed-readline

make

#chown -Rv tester .
#su -s /usr/bin/expect tester << EOF
#set timeout -1
#spawn make tests
#expect eof
#lassign [wait] _ _ _ value
#exit $value
#EOF
make install

#TODO:替换正在执行的版本
