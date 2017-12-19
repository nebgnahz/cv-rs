# cv-rs

[![Build Status][travis-image]][travis-url]
[![Build status][appveyor-image]][appveyor-url]
[![standard-readme compliant][standard-readme-image]][standard-readme-url]

> This library primarily provides bindings (somewhat idiomatic) and APIs for
> OpenCV 3.x.

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

Before anything, make sure you have OpenCV 3 installed. Read this
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
features = [ "gpu" ]
```

### Windows

Depending on your install, you might have to set `%OPENCV_DIR%` and
`%OPENCV_LIB%` environment variables such that `%OPENCV_DIR%\include` points to
the OpenCV includes diretory and `%OPENCV_LIB%` has `opencv_world320.lib`. You
will also need to update `%PATH%` for `opencv_world320.dll`.

## Usage

See available examples on how this library might be used.

- [Display Image](examples/display_image.rs)
- [Video Capture](examples/video_capture.rs), optional GPU code
- [Face Detection](examples/face_detect.rs)
- [Camshift](examples/camshift.rs)
- [HOG Detection](examples/hog.rs), optional GPU code

## Contribute

See [the contribute file](contribute.md)! PRs highly welcome.

You may also simply open up an issue for feature/porting request.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © Ben Zhang

<!-- links -->
[travis-image]: https://travis-ci.org/nebgnahz/cv-rs.svg?branch=master
[travis-url]: https://travis-ci.org/nebgnahz/cv-rs
[appveyor-image]: https://ci.appveyor.com/api/projects/status/dutogjshst3oyra2?svg=true
[appveyor-url]: https://ci.appveyor.com/project/nebgnahz/cv-rs
[standard-readme-image]: https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square
[standard-readme-url]: https://github.com/RichardLitt/standard-readme
[opencv-intro]: http://docs.opencv.org/3.1.0/df/d65/tutorial_table_of_content_introduction.html
