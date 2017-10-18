#!/bin/bash
set -eux -o pipefail

OPENCV_VERSION=${OPENCV_VERSION:-3.3.0}
URL=https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.zip

if [[ ! -e "$HOME/usr/installed-${OPENCV_VERSION}" ]]; then
    TMP=$(mktemp -d)
    if [[ ! -d "opencv-${OPENCV_VERSION}/build" ]]; then
        curl -sL ${URL}  > ${TMP}/opencv.zip
        unzip -q ${TMP}/opencv.zip
        mkdir opencv-${OPENCV_VERSION}/build
        rm ${TMP}/opencv.zip
    fi

    cd opencv-${OPENCV_VERSION}/build
    cmake -D BUILD_EXAMPLES=OFF \
          -D BUILD_TESTS=OFF \
          -D BUILD_PERF_TESTS=OFF  \
          -D BUILD_opencv_java=OFF \
          -D BUILD_opencv_python=OFF \
          -D BUILD_opencv_python2=OFF \
          -D BUILD_opencv_python3=OFF \
          -D CMAKE_INSTALL_PREFIX=$HOME/usr \
          ..
    make -j4
    make install && touch $HOME/usr/installed-${OPENCV_VERSION}
    cd ../..
    touch $HOME/fresh-cache
fi

cp $HOME/usr/include/* /usr/local/include
cp $HOME/usr/lib/* /usr/local/lib
