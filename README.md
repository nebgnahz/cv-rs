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
bindings to access OpenCV functionalities. First, C bindings are created
(in [native](native) folder); then [Rust APIs](src/lib.rs) are constructed
atop. Although this manual process seems an inefficient process, it has served
me well as a learning experience to both OpenCV and Rust. In terms of OpenCV API
coverage, modules and functions are implemented as needed.

Please check out the [documentation](https://nebgnahz.github.io/cv-rs/cv/) to
see what has been ported. If you have demand for porting specific features,
please open an issue, or better create a PR.

Attempts to use [rust-bindgen](https://github.com/servo/rust-bindgen)
or [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust) haven't been very
successful (I probably haven't tried hard enough). There is another
port [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
bindings using a Python script (more automated).

## Install

Before anything, make sure you have OpenCV 3 installed. If you are using windows, follow [this instruction](#windows), otherwise read this
[Introduction to OpenCV][opencv-intro] to get started.

Then in any Rust project, add this to your `Cargo.toml`:

```
[dependencies]
cv = { git = "https://github.com/nebgnahz/cv-rs.git" }
```

And add this to your crate:

```
extern crate cv;
use cv::*;
```

And then, enjoy the power of OpenCV.

If you'd like to use OpenCV GPU functions, it's inside `cv::cuda`. Enable it
with the following code in `Cargo.toml`:

```
[dependencies.cv]
git = "https://github.com/nebgnahz/cv-rs"
features = [ "cuda" ]
```

All possible features are listed below:
- `cuda` - for CUDA support, requires installed CUDA
- `tesseract` - for Tesseract OCR support, requires installed Tesseract

### Windows

#### If you are using MSVC toolchain (mandatory if you want to use CUDA)
##### Prerequisites
- Installed git.
- Installed CMake x64 ([download link](https://cmake.org/download/)).
- Installed Visual Studio 2015 ([download link](https://go.microsoft.com/fwlink/?LinkId=532606&clcid=0x409)), VS2017 is not supported by nVidia at this moment, don't even try, it won't compile.

##### Installation steps
- Create directory `C:\opencv`.
- Copy `.git` and `.windows` folders there (you can run them from the `cv-rs` directory itself, but you may encounter an error that paths are too long)
- Run powershell console as administrator in `c:\opencv`.
- (***Optional, skip these steps if you don't need CUDA***)
    1. Download CUDA from [official site](https://developer.nvidia.com/cuda-downloads?target_os=Windows&target_arch=x86_64&target_version=10). Choose `local` package.
    1. Run `PowerShell -NoExit -File .\.windows\msvc_1_install_CUDA.ps1 -FileName path_to_installer` (for example, `C:\Users\UserName\Downloads\cuda_9.1.85_win10.exe`).
- Run `PowerShell -NoExit -File (.\.windows\msvc_2_build_OCV.ps1 -EnableCuda $False -Compiler vc15)` (note braces). `1` stays for compilation with CUDA, `0` for compilation without it. Possible compiler values: `vc14` for VS2015/`vc15` for VS2017. **Caution: CUDA is compatible with VS2015 only**
- Wait until installation finishes. Now you have properly configured OpenCV.

#### If you are using GNU toolchain

##### Prerequisites
- Installed git.
- Installed CMake x64 ([download link](https://cmake.org/download/)).
- Installed MinGW ([download link](https://sourceforge.net/projects/mingw-w64/files/latest/download)). Choose architecture `x86_64` during installation.

##### Installation steps
- Create directory `C:\opencv`.
- Copy `.git` and `.windows` folders there (you can run them from the `cv-rs` directory itself, but you may encounter an error that paths are too long)
- Run powershell console as administrator in `c:\opencv`.
- Run `PowerShell -NoExit -File .\.windows\mingw_build_OCV.ps1 -MinGWPath "C:\Program Files\mingw-w64\x86_64-7.2.0-posix-seh-rt_v5-rev1\mingw64\bin"` (your path may be different).
- Wait until installation finishes. Now you have properly configured OpenCV.

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
