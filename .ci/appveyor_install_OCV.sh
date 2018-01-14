#!/bin/bash -e
myRepo=$(pwd)
CMAKE_CONFIG_GENERATOR="Visual Studio 15 2017 Win64"
CMAKE_OPTIONS='-DWITH_CUDA:BOOL=ON -DCUDA_ARCH_BIN="5.2" -DCUDA_ARCH_PTX="" -DBUILD_opencv_java:BOOL=OFF -DBUILD_opencv_python:BOOL=OFF -DBUILD_opencv_python2:BOOL=OFF -DBUILD_opencv_python3:BOOL=OFF -DBUILD_PERF_TESTS:BOOL=OFF -DBUILD_TESTS:BOOL=OFF -DBUILD_DOCS:BOOL=OFF -DBUILD_EXAMPLES:BOOL=OFF -DINSTALL_CREATE_DISTRIB:BOOL=ON'

if [  ! -d "$myRepo/opencv"  ]; then
    echo "clonning opencv"
    git clone https://github.com/opencv/opencv.git
else
    cd opencv
    git pull --rebase
    cd ..
fi
if [  ! -d "$myRepo/opencv_contrib"  ]; then
    echo "clonning opencv_contrib"
    git clone https://github.com/opencv/opencv_contrib.git
else
    cd opencv_contrib
    git pull --rebase
    cd ..
fi
RepoSource=opencv
pushd build/$RepoSource
cmake -G"$CMAKE_CONFIG_GENERATOR" $CMAKE_OPTIONS -DOPENCV_EXTRA_MODULES_PATH="$myRepo"/opencv_contrib/modules -DCMAKE_INSTALL_PREFIX="$myRepo"/install/"$RepoSource" "$myRepo/$RepoSource"
echo "************************* $Source_DIR -->release"
cmake --build .  --target install --config release
popd