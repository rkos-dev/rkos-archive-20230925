cat > /etc/sysconfig/ifconfig.eth0 << "EOF"
ONBOOT="yes"
IFACE="eth0"
SERVICE="dhcpcd"
DHCP_START="-b -q -h ''<insert appropriate start options here>"
DHCP_STOP="-k <insert additional stop options here>"
EOF

cat > /etc/resolv.conf << "EOF"
# Begin /etc/resolv.conf

nameserver 114.114.114.114
nameserver 8.8.8.8

# End /etc/resolv.conf
EOF


echo "rkos" > /etc/hostname


cat > /etc/hosts << "EOF"
# Begin /etc/hosts


127.0.0.1       localhost
127.0.1.1       rkos.localdomain localhost 
::1     localhost
ff02::1   ip6-allnodes
ff02::2   ip6-allrouters

# End /etc/hosts
EOF


