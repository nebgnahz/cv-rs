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

use std::os::raw::{c_char, c_void};
use std::mem;

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

extern "C" {
    fn c_drop(value: *mut c_void);
}

#[repr(C)]
struct CResult<T: Copy> {
    value: T,
    error: *const c_char,
}

impl<T: Copy> Into<Result<T, String>> for CResult<T> {
    fn into(self) -> Result<T, String> {
        if self.error.is_null() {
            Ok(self.value)
        } else {
            unsafe {
                let c_str = std::ffi::CStr::from_ptr(self.error);
                let err = c_str.to_string_lossy().into_owned();
                Err(err)
            }
        }
    }
}

impl<T: Copy> Drop for CResult<T> {
    fn drop(&mut self) {
        if !self.error.is_null() {
            unsafe { c_drop(self.error as *mut c_void) }
        }
    }
}

impl<T: Copy> CResult<T> {
    pub fn from_callback<F: FnOnce(*mut CResult<T>)>(func: F) -> CResult<T> {
        let mut result: CResult<T>;
        unsafe {
            result = mem::uninitialized();
            let result_ref: *mut CResult<T> = &mut result;
            func(result_ref);
        };
        result
    }
}