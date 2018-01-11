#!/bin/sh
## Install CUDA (must run as root)

CUDA_PKG_VERSION="7-5"
CUDA_VERSION="7.5"

CUDA_REPO_PKG=cuda-repo-ubuntu1404_${CUDA_VERSION}-18_amd64.deb
NVIDIA_URL=http://developer.download.nvidia.com/compute/cuda/repos/ubuntu1404/x86_64/
wget ${NVIDIA_URL}${CUDA_REPO_PKG}
dpkg -i ${CUDA_REPO_PKG}
rm ${CUDA_REPO_PKG}

## update and install package
apt-get -y update
apt-get install -y cuda-$CUDA_PKG_VERSION

## manually create CUDA symlink
ln -s /usr/local/cuda-${CUDA_VERSION} /usr/local/cuda
