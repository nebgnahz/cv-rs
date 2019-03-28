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
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate bytes;
#[macro_use]
extern crate failure;
extern crate native;

pub mod core;
#[cfg(feature = "cuda")]
pub mod cuda;
pub mod errors;
pub mod features2d;
pub mod hash;
pub mod highgui;
pub mod imgcodecs;
pub mod imgproc;
pub mod mat;
pub mod objdetect;
#[cfg(feature = "text")]
pub mod text;
pub mod video;
pub mod videoio;

pub use core::*;
pub use mat::*;

use errors::*;
use failure::Error;
use std::ffi::CString;
use std::mem;
use std::path::Path;

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Result<CString, Error> {
    let path = path.as_ref();
    let x = path.to_str().ok_or_else(|| CvError::InvalidPath(path.into()))?;
    let result = CString::new(x)?;
    Ok(result)
}
