#!/bin/sh
## Install CUDA (must run as root)
## install repo packages

CUDA_REPO_PKG=cuda-repo-ubuntu1404_7.5-18_amd64.deb
NVIDIA_URL=http://developer.download.nvidia.com/compute/cuda/repos/ubuntu1404/x86_64/
wget $NVIDIA_URL$CUDA_REPO_PKG
dpkg -i $CUDA_REPO_PKG
rm $CUDA_REPO_PKG

## update package lists
apt-get -y update

## install packages
CUDA_PKG_VERSION="7-5"
CUDA_VERSION="7.5"
apt-get install -y --no-install-recommends \
        cuda-core-$CUDA_PKG_VERSION \
        cuda-cudart-dev-$CUDA_PKG_VERSION \
        cuda-cublas-dev-$CUDA_PKG_VERSION \
        cuda-curand-dev-$CUDA_PKG_VERSION

## manually create CUDA symlink
ln -s /usr/local/cuda-$CUDA_VERSION /usr/local/cuda
