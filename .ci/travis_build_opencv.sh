#!/bin/bash
set -eux -o pipefail

OPENCV_VERSION=${OPENCV_VERSION:-3.4.0}
URL=https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.zip
URL_CONTRUB=https://github.com/opencv/opencv_contrib/archive/${OPENCV_VERSION}.zip

if [[ ! -e "$HOME/usr/installed-${OPENCV_VERSION}" ]]; then
    TMP=$(mktemp -d)
    if [[ ! -d "opencv-${OPENCV_VERSION}/build" ]]; then
        curl -sL ${URL}  > ${TMP}/opencv.zip
        unzip -q ${TMP}/opencv.zip
        rm ${TMP}/opencv.zip
        
        curl -sL ${URL_CONTRUB}  > ${TMP}/opencv_contrib.zip
        unzip -q ${TMP}/opencv_contrib.zip
        rm ${TMP}/opencv_contrib.zip
        
        mkdir opencv-${OPENCV_VERSION}/build
        
    fi

    cd opencv-${OPENCV_VERSION}/build
    cmake \
        -D WITH_CUDA=ON \
        -D BUILD_EXAMPLES=OFF \
        -D BUILD_TESTS=OFF \
        -D BUILD_PERF_TESTS=OFF  \
        -D BUILD_opencv_java=OFF \
        -D BUILD_opencv_python=OFF \
        -D BUILD_opencv_python2=OFF \
        -D BUILD_opencv_python3=OFF \
        -D CMAKE_INSTALL_PREFIX=$HOME/usr \
        -D CMAKE_BUILD_TYPE=Release \
        -D OPENCV_EXTRA_MODULES_PATH=opencv_contrib-${OPENCV_VERSION}/modules \
        -D CUDA_ARCH_BIN=5.2 \
        -D CUDA_ARCH_PTX="" \
        ..
    make -j4
    make install && touch $HOME/usr/installed-${OPENCV_VERSION}
    cd ../..
    touch $HOME/fresh-cache
fi

sudo cp -r $HOME/usr/include/* /usr/local/include/
sudo cp -r $HOME/usr/lib/* /usr/local/lib/
