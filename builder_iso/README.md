### 宿主机环境安装指南
- 将配置文件（user_configuration，user_disk_layout，user_credentials）copy到使用arch镜像启动的kvm虚拟机中
- 在配置文件所在目录执行命令```archinstall --config user_configuration.json --creds user_credentials.json --disk_layouts user_disk_layout.json```
- 无需做任何修改，选择install
- 安装完毕后不需要进入chroot环境，直接reboot
- 执行prepare_env.sh后重启系统
- 执行create_dirs.sh
- 执行version-check.sh
- 全部没有报错或者not found字样即配置成功
