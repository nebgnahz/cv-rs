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
use libc::{c_double, c_int};

pub mod highgui;

mod core;
use core::CMat;

#[cfg(feature = "gpu")]
pub mod cuda;

pub use core::CvType;
pub use core::FlipCode;
pub use core::LineTypes;
pub use core::Mat;
pub use core::Point2f;
pub use core::Point2i;
pub use core::Rect;
pub use core::Scalar;
pub use core::Size2f;
pub use core::Size2i;

// =============================================================================
//  core array
// =============================================================================
extern "C" {
    fn opencv_in_range(cmat: *const CMat, lowerb: Scalar, upperb: Scalar, dst: *mut CMat);
    fn opencv_mix_channels(cmat: *const CMat,
                           nsrcs: isize,
                           dst: *mut CMat,
                           ndsts: isize,
                           from_to: *const i32,
                           npairs: isize);
    fn opencv_normalize(csrc: *const CMat, cdst: *mut CMat, alpha: c_double, beta: c_double, norm_type: c_int);
}

/// Normalization type. Please refer to [OpenCV's
/// documentation](http://docs.opencv.org/trunk/d2/de8/group__core__array.html#gad12cefbcb5291cf958a85b4b67b6149f).
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum NormTypes {
    /// Normalized using `max`
    NormInf = 1,
    /// Normalized using L1 distance
    NormL1 = 2,
    /// Normalized using L2 distance
    NormL2 = 4,
    /// Normalized using L2 sqr distance
    NormL2Sqr = 5,
    /// Normalized using hamming distance
    NormHamming = 6,
    /// Normalized using hamming2 distance
    NormHamming2 = 7,
    /// Normalized using relative distance
    NormRelative = 8,
    /// Normalized using minmax distance
    NormMinMax = 32,
}

impl Mat {
    /// Check if Mat elements lie between the elements of two other arrays
    /// (lowerb and upperb). The output Mat has the same size as `self` and
    /// CV_8U type.
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = CMat::new();
        unsafe { opencv_in_range(self.inner, lowerb, upperb, m) }
        Mat::from_raw(m)
    }

    /// Copy specified channels from `self` to the specified channels of output
    /// `Mat`.
    // TODO(benzh) Avoid using raw pointers but rather take a vec for `from_to`?
    pub fn mix_channels(&self, nsrcs: isize, ndsts: isize, from_to: *const i32, npairs: isize) -> Mat {
        let m = Mat::with_size(self.rows, self.cols, self.depth);
        unsafe {
            opencv_mix_channels(self.inner, nsrcs, m.inner, ndsts, from_to, npairs);
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormTypes) -> Mat {
        let m = CMat::new();
        unsafe { opencv_normalize(self.inner, m, alpha, beta, t as i32) }
        Mat::from_raw(m)
    }
}

pub mod imgcodecs;
pub mod imgproc;
pub mod videoio;
pub mod objdetect;
pub mod video;
