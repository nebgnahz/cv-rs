#!/bin/bash
set -eux -o pipefail

OPENCV_VERSION=${OPENCV_VERSION:-3.4.0}
URL=https://github.com/opencv/opencv/archive/$OPENCV_VERSION.zip
URL_CONTRIB=https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.zip
OPENCV_BUILD=$(pwd)/opencv-$OPENCV_VERSION/build
OPENCV_CONTRIB=$(pwd)/opencv_contrib-$OPENCV_VERSION/modules
INSTALL_FLAG=$HOME/usr/installed-$OPENCV_VERSION
INSTALL_PREFIX=$HOME/usr

if [[ ! -e $INSTALL_FLAG ]]; then
    TMP=$(mktemp -d)
    if [[ ! -d $OPENCV_BUILD ]]; then
        curl -sL ${URL}  > ${TMP}/opencv.zip
        unzip -q ${TMP}/opencv.zip
        rm ${TMP}/opencv.zip

        curl -sL ${URL_CONTRIB}  > ${TMP}/opencv_contrib.zip
        unzip -q ${TMP}/opencv_contrib.zip
        rm ${TMP}/opencv_contrib.zip

        mkdir $OPENCV_BUILD
    fi

    pushd $OPENCV_BUILD
    cmake \
        -D WITH_CUDA=ON \
        -D BUILD_EXAMPLES=OFF \
        -D BUILD_TESTS=OFF \
        -D BUILD_PERF_TESTS=OFF  \
        -D BUILD_opencv_java=OFF \
        -D BUILD_opencv_python=OFF \
        -D BUILD_opencv_python2=OFF \
        -D BUILD_opencv_python3=OFF \
        -D OPENCV_EXTRA_MODULES_PATH=$OPENCV_CONTRIB \
        -D CMAKE_INSTALL_PREFIX=$INSTALL_PREFIX \
        -D CMAKE_BUILD_TYPE=Release \
        -D CUDA_ARCH_BIN=5.2 \
        -D CUDA_ARCH_PTX="" \
        ..
    make install && touch $INSTALL_FLAG
    popd
    touch $HOME/fresh-cache
fi

sudo cp -r $HOME/usr/include/* /usr/local/include/
sudo cp -r $HOME/usr/lib/* /usr/local/lib/
sudo ldconfig
