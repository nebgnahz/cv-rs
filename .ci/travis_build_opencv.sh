#!/bin/bash
set -eux -o pipefail

OPENCV_VERSION=${OPENCV_VERSION:-3.4.0}
URL=https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.zip
URL_CONTRIB=https://github.com/opencv/opencv_contrib/archive/${OPENCV_VERSION}.zip
CACHE=${HOME}/usr/installed-${OPENCV_VERSION}

if [[ ! -e ${CACHE} ]]; then
    TMP=$(mktemp -d)
    if [[ ! -d "opencv-${OPENCV_VERSION}/build" ]]; then
        curl -sL ${URL}  > ${TMP}/opencv.zip
        unzip -q ${TMP}/opencv.zip
        rm ${TMP}/opencv.zip

        curl -sL ${URL_CONTRIB}  > ${TMP}/opencv_contrib.zip
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
        -D OPENCV_EXTRA_MODULES_PATH=../../opencv_contrib-${OPENCV_VERSION}/modules \
        -D CMAKE_INSTALL_PREFIX=$HOME/usr \
        -D CUDA_ARCH_BIN=5.2 \
        -D CUDA_ARCH_PTX="" \
        ..
    make -j4
    make install && touch ${CACHE}
    cd ../..
fi

sudo cp -r $HOME/usr/include/* /usr/local/include/
sudo cp -r $HOME/usr/lib/* /usr/local/lib/
sudo ldconfig
