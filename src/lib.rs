//! This library primarily provides a binding and API for OpenCV 3.x.
//!
//! This is a work-in-progress and modules/functions are implemented as
//! needed. Attempts to use
//! [rust-bindgen](https://github.com/servo/rust-bindgen) or
//! [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust) haven't been very
//! successful (I probably haven't tried hard enough). There is another port
//! [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
//! bindings using a Python script.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate bytes;
#[macro_use]
extern crate failure;
extern crate num;
#[macro_use]
extern crate num_derive;

mod core;
pub use core::CvType;
pub use core::FlipCode;
pub use core::LineTypes;
pub use core::Mat;
pub use core::NormTypes;
pub use core::Point2f;
pub use core::Point2i;
pub use core::Rect;
pub use core::Scalar;
pub use core::Size2f;
pub use core::Size2i;

pub mod errors;
pub mod imgproc;
pub mod imgcodecs;
pub mod videoio;
pub mod highgui;
pub mod video;
pub mod objdetect;
pub mod features2d;

#[cfg(feature = "gpu")]
pub mod cuda;
