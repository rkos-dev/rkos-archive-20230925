# rkos
Rust King OS - Linux Distro of Rust Programing Language

# Tutorial

## Host preparation

### ATTENTION!!!

- rkos is currently under active development and is only suitable for development and testing, and there is a risk of damage to the host machine where rkos is being built.

- Please make sure you are currently using the root user, and you should use the root user for all build processes.

- Please confirm that the total size of the memory and swap partition is 32GB or more. If you use the provided host environment, please ensure that the available memory of the host is greater than or equal to 32GB or that the third disk has sufficient capacity and is mounted as a swap partition.

- Warn:
    - In theory, building the program will not destroy the host environment, but there may be unexpected bugs. For safety reasons, it is recommended to use a virtual machine as the host.

    - Without carefully checking the code of the script, manually executing any script may cause the host environment to be destroyed.




### Use the provided host environment.

- Run version-check.sh to view the output and determine whether there are problems, such as missing software packages. If missing, please install manually.

- Build the host system and use kvm to start the host system.

- Mount the swap partition.
    ```
    mkswap /dev/vdc1
    swapon /dev/vdc1
    ```

- Compile and build tools.
    ```
    git clone https://github.com/open-rust-initiative/rkos
    cd ./rkos
    cargo build --release
    ```

- Place the configuration files (configs, scripts, config-6.1, and umount.sh) and the running program (rkos-builder) under the host system target partition (/mnt/lfs/).
    ```
    cp -r src/configs src/scripts src/config-6.1 src/umount.sh target/release/rkos-builder /mnt/lfs/
    ```
- Run rkos-builder --help to view the instructions and build according to the option process.

- After the build is complete, compress the target partition qcow2 image on the host, and the image directory should be at the location where the host kvm image is stored.

    ```
    TMPDIR=/home/tmp/path virt-sparsity --compress xxx.qcow2 xxx_compress.qcow2
    ```

### Use your own Linux

- Install the necessary software for the host.
    ```
    Bash >= 3.2 (/bin/sh must be a symlink or hardlink to bash)
    Binutils >= 2.13.1
    Bison >= 2.7 (/usr/bin/yacc must be a link to bison, or a small script that executes bison)
    Coreutils >= 6.9
    Diffutils >= 2.8.1
    Findutils >= 4.2.31
    Gawk >= 4.0.1 (/usr/bin/awk must be a link to gawk)
    GCC >= 4.8, including the C++ compiler g++, and the C and C++ standard libraries (including header files) must also be available so that the C++ compiler can build programs for the host environment.
    Grep >= 2.5.1a
    Gzip >= 1.3.12
    Linux Kernel >= 3.2
    M4 >= 1.4.10
    Make >= 4.0
    Patch >= 2.5.4
    Perl >= 5.8.8
    Python >= 3.4
    Sed >= 4.1.5
    Tar >= 1.22
    Texinfo >= 4.7
    Xz >= 5.0.0
    rust
    git
    clang
    parted
    pkg-config
    ```

- If it is arch, please install the arch development kit ```pacman -S base-devel```

- Run version-check.sh to view the output and determine whether there are problems, such as missing software packages.
    - Note: rust git clang parted pkg-config will not be checked, so you should make sure these packages are installed.

- Install QEMU image-related tools.

    ```
    sudo pacman -S qemu-img nbd
    ```

- Create a qcow2 image with a size of 30G.

    ```
    qemu-img create -f qcow2 vda.qcow2 30G
    ```

- Set up partitions and mount.

    ```
    sudo modprobe nbd max_part=16

    # Please change the x in nbdx to an actual number. 
    qemu-nbd -c /dev/nbdx vda.qocw2

    sudo parted /dev/nbdx mklabel msdos
    sudo parted /dev/nbdx mkpart primary fat32 0% 200M
    sudo parted /dev/nbdx mkpart primary ext4 200M 100%

    sudo mkfs.vfat /dev/nbdxp1
    sudo mkfs.ext4 /dev/nbdxp2

    mkdir /mnt/lfs
    sudo mount /dev/nbdxp2 /mnt/lfs

    mkdir /mnt/lfs/boot
    sudo mount /dev/nbdxp1 /mnt/lfs/boot

    ```
- Adjust configuration files (adjust according to needs).

    - File path: configs/base_configs.json

    - Set "path": "install_path": to /mnt/lfs/ or other paths, but make sure it is in the /mnt directory."

    - Set "envs": ["to the same path as above"]

    - Change /dev/vdb1 and /dev/vdb2 in the script file scripts/prepare/prepare_host_env.sh to the actual partition used.

    - Change /dev/vdb in the script file scripts/sysconfig/config_grub.sh to the actual partition used.

    - Change /mnt/lfs in the script file umount.sh to the actual mount point used.

    Note that there needs to be a '/' symbol at the end of the path.


- Place the configuration files (configs, scripts, config-6.1, and umount.sh) and the running program (rkos-builder) under the host system target partition (/mnt/lfs/).

    ```
    cp -r rkos/src/configs rkos/src/config-6.1 rkos/src/umount.sh rkos/src/scripts /mnt/lfs/
    ```

- You can directly execute the automated build process. Run ```rkos-builder build start``` If there is an error during the process, please check the log and delete all mounts (run .umount.sh) and delete all files on the disk, restart after troubleshooting.

- Or You can run rkos-builder --help to view the instructions, and build according to the option process. After each process is executed, be sure to run./umount.sh, and the output of this script can not be ignored.
    - Process：
        - host-config
        - package-download
        - build-temp-toolchains
        - build-base-packages
        - config-target-system
        - build-rust-support-package-and-kernel
        - install-grub


- After the build is complete, compress the target partition qcow2 image on the host, and the image directory should be at the location where the host kvm image is stored.

    ```
    qemu-nbd -d /mnt/lfs 

    sudo pacman -S guestfs-tools
    TMPDIR=/home/tmp/path virt-sparsity --compress xxx.qcow2 xxx_compress.qcow2
    ```

# Command

```
rkos-builder --help

Usage: rkos-builder [OPTIONS] <BUILD_OPTION> <OPERATE> [PACKAGE_NAME]

Arguments:

    <BUILD_OPTION>  possible values: 
                    build,
                    host-config,
                    package-download,
                    build-temp-toolchains,
                    build-base-packages,
                    config-target-system,
                    build-rust-support-package-and-kernel,
                    install-grub,
                    clean-up        #Comming soon

    <OPERATE>       possible values:
                    start,
                    reset           #Comming soon

    <PACKAGE_NAME>  default:NULL    #Comming soon

Options:
    -c, --config <DIR>              #Comming soon
    -d, --debug...                  #Comming soon
    -h, --help      Print help
    -V, --version   PrintVersion
```
## Example：

    ```
    # Configure the host environment.
    rkos-builder host-config start 
    ```

- When building, you can manually build each step from host-config according to the options in BUILD_OPTION>. After build-temp-toolchains, you need to manually run umount.sh after each step is completed and then start the next step.

- build-base-packages needs to build clang; it may take more than 2 hours depending on the performance of the machine, and the host needs to be mounted with a 20GB swap partition.

- You can directly use the build option to build all processes. It is not stable yet, and problems may occur. The build log will be recorded in the root/prepare.log, root/config.log, and root/log.log files of the target partition. Record the logs in the preparation process of the host environment, the logs in the configuration, and the logs in the construction process.

# Issue

- The e2fsck version is too old; there will be an error when the kernel starts; it will not affect startup; it will be fixed soon.

- Host network failure will lead to configuration and installation failure.

- The log output of the build and installation processes of rust packages (rust, coreutils, and kernel) will not be recorded and will be fixed soon.

- The download of rust-src may fail due to network instability; just restart the corresponding option.

- The software package installation verification function is lacking, and there may be missing installations of the software package. Currently, there may be 1-2 missing package installations in the test.

- If there is a problem during the installation process, after fixing the problem, temporarily delete the installed software package from the corresponding configuration file to continue building.

    ```
    cp configs/[package_info.json|rust_support_packages.json]{,.bak}

    # Then find the corresponding software package that failed to install, delete all the previously installed ones, and after solving the problem of failed installation, you can continue to run the build process.
    ```

# Contributing

rkos is a linux distro of rust programing language. The project relies on community contributions and aims to simplify getting started. To use rkos, clone the repo, install dependencies, and run rkso-builder. Pick an issue, make changes, and submit a pull request for community review.

To contribute to rkos, you should:

- Familiarize yourself with the [Code of Conduct](CODE-OF-CONDUCT.md). rkos has a strict policy against abusive, unethical, or illegal behavior.
- Review the [Contributing Guidelines](CONTRIBUTING.md). This document outlines the process for submitting bug reports, feature requests, and pull requests to rkos.
- Sign the [Developer Certificate of Origin](https://developercertificate.org) (DCO) by adding a `Signed-off-by` line to your commit messages. This certifies that you wrote or have the right to submit the code you are contributing to the project.
- Choose an issue to work on. Issues labeled `good first issue` are suitable for newcomers. You can also look for issues marked `help wanted`.
- Fork the rkos repository and create a branch for your changes.
- Make your changes and commit them with a clear commit message.
- Push your changes to GitHub and open a pull request.
- Respond to any feedback on your pull request. The rkos maintainers will review your changes and may request modifications before merging.
- Once your pull request is merged, you will be listed as a contributor in the project repository and documentation.

To comply with the requirements, contributors must include both a `Signed-off-by` line and a PGP signature in their commit messages. You can find more information about how to generate a PGP key [here](https://docs.github.com/en/github/authenticating-to-github/managing-commit-signature-verification/generating-a-new-gpg-key).

Git even has a `-s` command line option to append this automatically to your commit message, and `-S` to sign your commit with your PGP key. For example:

```bash
$ git commit -S -s -m 'This is my commit message'
```

## Rebase the branch

If you have a local git environment and meet the criteria below, one option is to rebase the branch and add your Signed-off-by lines in the new commits. Please note that if others have already begun work based upon the commits in this branch, this solution will rewrite history and may cause serious issues for collaborators (described in the git documentation under “The Perils of Rebasing”).

You should only do this if:

- You are the only author of the commits in this branch
- You are absolutely certain nobody else is doing any work based upon this branch
- There are no empty commits in the branch (for example, a DCO Remediation Commit which was added using `-allow-empty`)

To add your Signed-off-by line to every commit in this branch:

- Ensure you have a local copy of your branch by checking out the pull request locally via command line.
- In your local branch, run: `git rebase HEAD~1 --signoff`
- Force push your changes to overwrite the branch: `git push --force-with-lease origin main`

# License
rkos is licensed under this licensed:

- MIT LICENSE ( [LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
