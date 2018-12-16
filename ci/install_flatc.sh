#!/bin/bash
set -eux

if [[ ! -d flatbuffers ]]; then
  git clone http://github.com/google/flatbuffers
fi
mkdir -p build
pushd build
cmake ../flatbuffers              \
  -DCMAKE_INSTALL_PREFIX=/usr     \
  -DCMAKE_INSTALL_LIBDIR=lib      \
  -DFLATBUFFERS_BUILD_FLATLIB=OFF \
  -DFLATBUFFERS_BUILD_SHAREDLIB=ON
make -j $(nproc)
make install
