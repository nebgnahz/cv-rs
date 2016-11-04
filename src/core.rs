//! Core data structures in OpenCV
extern crate libc;
extern crate std;

use libc::{c_int, c_double};
use std::ffi::CString;
use std::os::raw::c_char;

// Opaque data struct for C bindings
pub enum CMat {}

/// This wraps OpenCV's `Mat` class which is designed for n-dimensional dense
/// array. It's the most widely used data structure in image/video processing
/// since images are often stored as `Mat`.
#[derive(Debug)]
pub struct Mat {
    pub inner: *mut CMat,
    pub cols: i32,
    pub rows: i32,
    pub depth: i32,
}

/// A 4-element struct that is widely used to pass pixel values.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Scalar {
    pub v0: i32,
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
}

impl Scalar {
    pub fn new(v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
        Scalar {
            v0: v0,
            v1: v1,
            v2: v2,
            v3: v3,
        }
    }
}

/// 2D integer points specified by its coordinates `x` and `y`.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2i {
    pub x: i32,
    pub y: i32,
}

/// 2D floating points specified by its coordinates `x` and `y`.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2f {
    pub x: f32,
    pub y: f32,
}

/// `Size2i` struct is used for specifying the size (`width` and `height` as
/// `i32`) of an image or rectangle.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2i {
    pub width: i32,
    pub height: i32,
}

impl Size2i {
    pub fn new(width: i32, height: i32) -> Self {
        Size2i {
            width: width,
            height: height,
        }
    }
}

/// `Size2f` struct is used for specifying the size (`width` and `height` as
/// `f32`) of an image or rectangle.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2f {
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
pub struct CVecOfRect {
    pub array: *mut Rect,
    pub size: usize,
}

impl Default for CVecOfRect {
    fn default() -> Self {
        CVecOfRect {
            array: std::ptr::null_mut::<Rect>(),
            size: 0,
        }
    }
}

impl Drop for CVecOfRect {
    fn drop(&mut self) {
        extern "C" {
            fn opencv_vec_of_rect_drop(_: *mut CVecOfRect);
        }
        unsafe {
            opencv_vec_of_rect_drop(self);
        }
    }
}

impl CVecOfRect {
    pub fn rustify(self) -> Vec<Rect> {
        (0..self.size)
            .map(|i| unsafe { *(self.array.offset(i as isize)) as Rect })
            .collect::<Vec<_>>()
    }
}

#[repr(C)]
pub struct CVecDouble {
    array: *mut c_double,
    size: usize,
}


impl CVecDouble {
    pub fn rustify(self) -> Vec<f64> {
        (1..self.size)
            .map(|i| unsafe { *(self.array.offset(i as isize)) as f64 })
            .collect::<Vec<_>>()
    }
}

impl Default for CVecDouble {
    fn default() -> Self {
        CVecDouble {
            array: std::ptr::null_mut::<c_double>(),
            size: 0,
        }
    }
}

extern "C" {
    pub fn opencv_mat_new() -> *mut CMat;
    fn opencv_mat_new_with_size(rows: i32, cols: i32, t: i32) -> *mut CMat;
    fn opencv_mat_is_valid(mat: *mut CMat) -> bool;
    fn opencv_mat_rows(cmat: *const CMat) -> i32;
    fn opencv_mat_cols(cmat: *const CMat) -> i32;
    fn opencv_mat_depth(cmat: *const CMat) -> i32;
    fn opencv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn opencv_mat_roi(cmat: *const CMat, rect: Rect) -> *mut CMat;
    fn opencv_mat_logic_and(cimage: *mut CMat, cmask: *const CMat);
    fn opencv_mat_flip(src: *mut CMat, code: i32);
    fn opencv_mat_drop(mat: *mut CMat);
}

pub enum FlipCode {
    XAxis,
    YAxis,
    XYAxis,
}

impl Mat {
    #[inline]
    pub fn new_with_cmat(cmat: *mut CMat) -> Mat {
        Mat {
            inner: cmat,
            rows: unsafe { opencv_mat_rows(cmat) },
            cols: unsafe { opencv_mat_cols(cmat) },
            depth: unsafe { opencv_mat_depth(cmat) },
        }
    }

    /// Create an empty `Mat` struct.
    pub fn new() -> Mat {
        let m = unsafe { opencv_mat_new() };
        Mat::new_with_cmat(m)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn with_size(rows: i32, cols: i32, t: i32) -> Self {
        let m = unsafe { opencv_mat_new_with_size(rows, cols, t) };
        Mat::new_with_cmat(m)
    }

    /// Create a `Mat` from reading the image specified by the path.
    pub fn from_path(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { opencv_imread((&s).as_ptr(), flags) };
        Mat::new_with_cmat(m)
    }

    /// Check if the `Mat` is valid or not.
    pub fn is_valid(&self) -> bool {
        unsafe { opencv_mat_is_valid(self.inner) }
    }

    /// Return a region of interest from a `Mat` specfied by a `Rect`.
    pub fn roi(&self, rect: Rect) -> Mat {
        let cmat = unsafe { opencv_mat_roi(self.inner, rect) };
        Mat::new_with_cmat(cmat)
    }

    /// Apply a mask to myself.
    // TODO(benzh): Find the right reference in OpenCV for this one. Provide a
    // shortcut for `image &= mask`
    pub fn logic_and(&mut self, mask: Mat) {
        unsafe {
            opencv_mat_logic_and(self.inner, mask.get_cmat());
        }
    }

    /// Flips an image around vertical, horizontal, or both axes.
    pub fn flip(&mut self, code: FlipCode) {
        let code = match code {
            FlipCode::XAxis => 0,
            FlipCode::YAxis => 1,
            FlipCode::XYAxis => -1,
        };
        unsafe {
            opencv_mat_flip(self.inner, code);
        }
    }

    /// Call out to highgui to show the image, the duration is specified by
    /// `delay`.
    pub fn show(&self, name: &str, delay: i32) {
        extern "C" {
            fn opencv_imshow(name: *const c_char, cmat: *mut CMat);
            fn opencv_wait_key(delay_ms: c_int) -> c_int;
        }

        let s = CString::new(name).unwrap();
        unsafe {
            opencv_imshow((&s).as_ptr(), self.inner);
            opencv_wait_key(delay);
        }
    }

    pub fn get_cmat(&self) -> *mut CMat {
        self.inner
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            opencv_mat_drop(self.inner);
        }
    }
}
