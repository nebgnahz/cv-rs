//! This library primarily provides a binding and API for OpenCV 3.1.0.
//!
//! This is a work-in-progress and modules/functions are implemented as
//! needed. Attempts to use
//! [rust-bindgen](https://github.com/servo/rust-bindgen) or
//! [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust) haven't been very
//! successful (I probably haven't tried hard enough). There is another port
//! [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
//! bindings using a Python script.
#![feature(proc_macro)]
#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

extern crate num;
#[macro_use]
extern crate num_derive;
extern crate libc;

mod core;
pub use core::CvType;
pub use core::FlipCode;
pub use core::LineTypes;
pub use core::NormTypes;
pub use core::Mat;
pub use core::Point2f;
pub use core::Point2i;
pub use core::Rect;
pub use core::Scalar;
pub use core::Size2f;
pub use core::Size2i;

pub mod imgproc;
pub mod imgcodecs;
pub mod videoio;
pub mod highgui;
pub mod video;
pub mod objdetect;

#[cfg(feature = "gpu")]
pub mod cuda;
