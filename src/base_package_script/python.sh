./configure --prefix=/usr        \
            --enable-shared      \
            --with-system-expat  \
            --with-system-ffi    \
            --enable-optimizations

make && make install

cat > /etc/pip.conf << EOF
[global]
root-user-action = ignore
disable-pip-version-check = true
EOF

#install -v -dm755 /usr/share/doc/python-3.10.6/html

#tar --strip-components=1  \
#    --no-same-owner       \
#    --no-same-permissions \
#    -C /usr/share/doc/python-3.10.6/html \
#    -xvf ../python-3.10.6-docs-html.tar.bz2
