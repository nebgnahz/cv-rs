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
#for test
 cat /var/lib/apt/lists/*cuda*Packages | grep "Package:"
#end
CUDA_PACKAGES="cuda-drivers cuda-core-${CUDA_VERSION} cuda-cublas-dev-${CUDA_VERSION} cuda-cudart-dev-${CUDA_VERSION} cuda-cufft-dev-${CUDA_VERSION} cuda-npp-dev-${CUDA_VERSION}"
apt-get -y update
apt-get install -y ${CUDA_PACKAGES}

## manually create CUDA symlink
ln -s /usr/local/cuda-${CUDA_VERSION} /usr/local/cuda
