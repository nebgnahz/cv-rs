#!/bin/bash
set -eux -o pipefail

OPENCV_VERSION=${OPENCV_VERSION:-3.4.0}
URL=https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.zip
URL_CONTRUB=https://github.com/opencv/opencv_contrib/archive/${OPENCV_VERSION}.zip

rm -rf "$HOME/usr/installed-${OPENCV_VERSION}"

if [[ ! -e "$HOME/usr/installed-${OPENCV_VERSION}" ]]; then
    TMP=$(mktemp -d)
    OPENCV_DIR="$(pwd)/opencv-${OPENCV_VERSION}"
    OPENCV_CONTRIB_DIR="$(pwd)/opencv_contrib-${OPENCV_VERSION}"
    if [[ ! -d "${OPENCV_DIR}/build" ]]; then
        curl -sL ${URL}  > ${TMP}/opencv.zip
        unzip -q ${TMP}/opencv.zip
        rm ${TMP}/opencv.zip

        curl -sL ${URL_CONTRUB}  > ${TMP}/opencv_contrib.zip
        unzip -q ${TMP}/opencv_contrib.zip
        rm ${TMP}/opencv_contrib.zip

        mkdir $OPENCV_DIR/build
    fi

    pushd $OPENCV_DIR/build
    cmake \
        -DWITH_CUDA=ON \
        -DBUILD_EXAMPLES=OFF \
        -DBUILD_TESTS=OFF \
        -DBUILD_PERF_TESTS=OFF  \
        -DBUILD_opencv_java=OFF \
        -DBUILD_opencv_python=OFF \
        -DBUILD_opencv_python2=OFF \
        -DBUILD_opencv_python3=OFF \
        -DCMAKE_INSTALL_PREFIX=$HOME/usr \
        -DCUDA_ARCH_BIN=5.2 \
        -DCUDA_ARCH_PTX="" \
        -DCMAKE_BUILD_TYPE=Release
        -DOPENCV_EXTRA_MODULES_PATH=$OPENCV_CONTRIB_DIR/modules \
        $OPENCV_DIR
    make -j4
    make install && touch $HOME/usr/installed-${OPENCV_VERSION}
    popd
    touch $HOME/fresh-cache
fi

sudo cp -r $HOME/usr/include/* /usr/local/include/
sudo cp -r $HOME/usr/lib/* /usr/local/lib/
