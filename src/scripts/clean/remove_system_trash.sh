rm -rf /tmp/*

find /usr/lib /usr/libexec -name \*.la -delete

find /usr -depth -name $(uname -m)-rkos-linux-gnu\* | xargs rm -rf

