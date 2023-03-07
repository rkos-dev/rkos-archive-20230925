tar -xf ../llvm-cmake-15.0.7.src.tar.xz &&
sed '/LLVM_COMMON_CMAKE_UTILS/s@../cmake@cmake-15.0.7.src@' \
    -i CMakeLists.txt

tar -xf ../clang-15.0.7.src.tar.xz -C tools &&
mv tools/clang-15.0.7.src tools/clang

tar -xf ../lld-15.0.7.src.tar.xz -C tools &&
mv tools/lld-15.0.7.src tools/lld

grep -rl '#!.*python' | xargs sed -i '1s/python$/python3/'
patch -Np2 -d tools/clang <../base_patches/clang-15.0.7-enable_default_ssp-1.patch

mkdir -v build &&
cd       build &&

CC=gcc CXX=g++                                  \
cmake -DCMAKE_INSTALL_PREFIX=/usr               \
      -DLLVM_ENABLE_FFI=ON                      \
      -DCMAKE_BUILD_TYPE=Release                \
      -DLLVM_BUILD_LLVM_DYLIB=ON                \
      -DLLVM_LINK_LLVM_DYLIB=ON                 \
      -DLLVM_ENABLE_RTTI=ON                     \
      -DLLVM_TARGETS_TO_BUILD="host;AMDGPU;BPF" \
      -DLLVM_BINUTILS_INCDIR=/usr/include       \
      -DLLVM_INCLUDE_BENCHMARKS=OFF             \
      -DCLANG_DEFAULT_PIE_ON_LINUX=ON           \
      -Wno-dev -G Ninja ..                      &&
ninja

ninja docs-clang-html docs-clang-man

ninja install &&
cp bin/FileCheck /usr/bin
