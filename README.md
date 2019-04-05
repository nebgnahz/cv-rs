# cv-rs

[![Build Status][travis-image]][travis-url]
[![Build status][appveyor-image]][appveyor-url]
[![standard-readme compliant][standard-readme-image]][standard-readme-url]

This library primarily provides idiomatic bindings and APIs for OpenCV 3.x.

[Documentation](https://nebgnahz.github.io/cv-rs/cv/)

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Contribute](#contribute)
- [License](#license)

## Background

OpenCV (Open Source Computer Vision Library: http://opencv.org) is an
open-source BSD-licensed library that includes several hundreds of computer
vision algorithms. It's mainly developed in C++. This library provides Rust
bindings to access OpenCV functionalities. First, C++ bindings are created
(in [cv-sys/native](cv-sys/native) folder), then [Rust APIs](src/lib.rs) are constructed
atop. In terms of OpenCV API coverage, modules and functions are implemented as needed.

To add new APIs, install `clang` on your system and add your APIs in
[cv-sys/native](cv-sys/native) to the appropriate module's `hpp` and `cc` files.
Then use the `gen-bindings` feature at the command line:
`cargo test -vv --features gen-bindings` (the `vv` lets you see
C++ compiler output). This will generate new bindings instead
of using the prebuilt ones. At this point you can now access the generated Rust
bindings (use `cargo doc -p cv-sys --features gen-bindings` to generate docs).
Use these bindings to implement native Rust bindings in [src](src). See the
existing bindings for examples.

To add new modules, please see [the build script](cv-sys/build.rs).

Please check out the [documentation](https://nebgnahz.github.io/cv-rs/cv/) to
see what has been ported. If you have demand for porting specific features,
please open an issue, or better create a PR.

There is another port [opencv-rust](https://github.com/kali/opencv-rust/) which
generates OpenCV bindings using a Python script (more automated). This binds
to OpenCV 2, so if you are looking for OpenCV 2, this could be more suitable for
your needs.

## Static Build (default behavior)

### Debian & Ubuntu

You must install these packages (use `sudo apt install <package>`):

- `clang`
- `libc++-dev`
- `cmake`
- `libgtk-3-dev`
- `libpng-dev`
- `libtiff-dev`
- `libjpeg-dev`
- `pkg-config`
- `libopenexr-dev`

#### `cuda` feature

- Install `nvidia-cuda-toolkit`
  - `sudo apt install nvidia-cuda-toolkit`
- Install `gcc-7`
- Install `g++-7`
- If `gcc-7` and `g++-7` are not the default (run `gcc -v` to check):
  - You can avoid these steps by uninstalling every `gcc` and `g++` aside from `7`.
  - `sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-7 7`
  - `sudo update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-7 7`
  - `sudo update-alternatives --config gcc`
    - Choose `gcc-7` manual option.
  - `sudo update-alternatives --config g++`
    - Choose `g++-7` manual option.
  - You may need to install alternatives for other `gcc` and `g++` verisons on your system
    if you wish to switch to another `gcc` version in the future.


Please create an issue if there are any other unlisted dependencies.

### Windows

You must have `cmake` installed.

## Usage

See available examples on how this library might be used.

- [Display Image](examples/display_image.rs)
- [Video Capture](examples/video_capture.rs), optional GPU code
- [Face Detection](examples/face_detect.rs)
- [Camshift](examples/camshift.rs)
- [HOG Detection](examples/hog.rs), optional GPU code

## Contribute

See [the contribute file](CONTRIBUTING.md)! PRs highly welcome.

You may also simply open up an issue for feature/porting request.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © Ben Zhang

<!-- links -->
[travis-image]: https://travis-ci.org/nebgnahz/cv-rs.svg?branch=master
[travis-url]: https://travis-ci.org/nebgnahz/cv-rs
[appveyor-image]: https://ci.appveyor.com/api/projects/status/dutogjshst3oyra2/branch/master?svg=true
[appveyor-url]: https://ci.appveyor.com/project/nebgnahz/cv-rs
[standard-readme-image]: https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square
[standard-readme-url]: https://github.com/RichardLitt/standard-readme
[opencv-intro]: http://docs.opencv.org/3.1.0/df/d65/tutorial_table_of_content_introduction.html
