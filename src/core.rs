//! Core data structures in OpenCV

use libc::{c_int, c_double, c_char, c_uchar, size_t};
use num;
use std::ffi::CString;

/// Opaque data struct for C bindings
#[derive(Clone, Copy, Debug)]
pub enum CMat {}
impl CMat {
    pub fn new() -> *mut CMat {
        unsafe { cv_mat_new() }
    }
}

/// This wraps OpenCV's `Mat` class which is designed for n-dimensional dense
/// array. It's the most widely used data structure in image/video processing
/// since images are often stored as `Mat`.
#[derive(Debug)]
pub struct Mat {
    /// Pointer to the actual C/C++ data structure
    pub inner: *mut CMat,

    /// Number of columns
    pub cols: i32,

    /// Number of rows
    pub rows: i32,

    /// Depth of this mat
    pub depth: i32,
}

/// A 4-element struct that is widely used to pass pixel values.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Scalar {
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
}

impl Scalar {
    /// Creates a new scalar object.
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
    /// x coordinate
    pub x: i32,

    /// y coordinate
    pub y: i32,
}

/// 2D floating points specified by its coordinates `x` and `y`.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2f {
    /// x coordinate
    pub x: f32,

    /// y coordinate
    pub y: f32,
}

/// `Size2i` struct is used for specifying the size (`width` and `height` as
/// `i32`) of an image or rectangle.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2i {
    /// width
    pub width: i32,

    /// height
    pub height: i32,
}

impl Size2i {
    /// Creates a new `Size2i` object with `width` and `height`
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
    /// width
    pub width: f32,

    /// height
    pub height: f32,
}

/// The `Rect` defines a rectangle in integer.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    /// x coordinate of the left-top corner
    pub x: i32,
    /// y coordinate of the left-top corner
    pub y: i32,
    /// width of this rectangle
    pub width: i32,
    /// height of this rectangle
    pub height: i32,
}

impl Rect {
    /// Creates a new `Rect` with (x, y, width, height) parameters.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// Scales the rectangle by the specified ratio.
    pub fn scale(&self, ratio: f32) -> Rect {
        let new_x = ((1.0 - ratio) * (self.width as f32) / 2.0) as i32 + self.x;
        let new_y = ((1.0 - ratio) * (self.height as f32) / 2.0) as i32 + self.y;
        let new_w = ((self.width as f32) * ratio) as i32;
        let new_h = ((self.height as f32) * ratio) as i32;
        Rect {
            x: new_x,
            y: new_y,
            width: new_w,
            height: new_h,
        }
    }

    /// Normalize the rectangle according to the image (if the rectangle is
    /// inside the image, then the result should be all within (0, 1).
    pub fn normalize_to_mat(&self, mat: &Mat) -> Rect2f {
        Rect2f {
            x: (self.x as f32) / (mat.cols as f32),
            y: (self.y as f32) / (mat.rows as f32),
            width: (self.width as f32) / (mat.cols as f32),
            height: (self.height as f32) / (mat.rows as f32),
        }
    }
}

/// The `Rect2f` are rectangles in float.
#[derive(Default, Debug, Clone, Copy)]
pub struct Rect2f {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect2f {
    /// Normalize the rectangle according to the image. This will restore the
    /// Rect in absolute pixel numbers.
    pub fn normalize_to_mat(&self, mat: &Mat) -> Rect {
        Rect {
            x: (self.x * mat.cols as f32) as i32,
            y: (self.y * mat.rows as f32) as i32,
            width: (self.width * mat.cols as f32) as i32,
            height: (self.height * mat.rows as f32) as i32,
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
            array: ::std::ptr::null_mut::<Rect>(),
            size: 0,
        }
    }
}

impl Drop for CVecOfRect {
    fn drop(&mut self) {
        extern "C" {
            fn cv_vec_of_rect_drop(_: *mut CVecOfRect);
        }
        unsafe {
            cv_vec_of_rect_drop(self);
        }
    }
}

impl CVecOfRect {
    pub fn rustify(self) -> Vec<Rect> {
        (0..self.size)
            .map(|i| unsafe { *(self.array.offset(i as isize)) })
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
            .map(|i| unsafe { *(self.array.offset(i as isize)) })
            .collect::<Vec<_>>()
    }
}

impl Default for CVecDouble {
    fn default() -> Self {
        CVecDouble {
            array: ::std::ptr::null_mut::<c_double>(),
            size: 0,
        }
    }
}

/// Line type
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineTypes {
    /// Default type
    Filled = -1,

    /// 4-connected line
    Line4 = 4,

    /// 8-connected line
    Line8 = 8,

    /// antialiased line
    LineAA = 16,
}

extern "C" {
    fn cv_mat_new() -> *mut CMat;
    fn cv_mat_new_with_size(rows: c_int, cols: c_int, t: i32) -> *mut CMat;
    fn cv_mat_is_valid(mat: *mut CMat) -> bool;
    fn cv_mat_rows(cmat: *const CMat) -> c_int;
    fn cv_mat_cols(cmat: *const CMat) -> c_int;
    fn cv_mat_depth(cmat: *const CMat) -> c_int;
    fn cv_mat_data(cmat: *const CMat) -> *const c_uchar;
    fn cv_mat_total(cmat: *const CMat) -> size_t;
    fn cv_mat_elem_size(cmat: *const CMat) -> size_t;
    fn cv_mat_type(cmat: *const CMat) -> c_int;
    fn cv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn cv_mat_roi(cmat: *const CMat, rect: Rect) -> *mut CMat;
    fn cv_mat_logic_and(cimage: *mut CMat, cmask: *const CMat);
    fn cv_mat_flip(src: *mut CMat, code: c_int);
    fn cv_mat_drop(mat: *mut CMat);
}

/// A flag to specify how to flip the image. see
/// [Mat::flip](struct.Mat.html#method.flip)
#[derive(Debug, Clone, Copy)]
pub enum FlipCode {
    /// Along x-axis: dst[i, j] = src[src.rows - i - 1, j]
    XAxis,
    /// Along y-axis: dst[i, j] = src[i, src.cols - j - 1]
    YAxis,
    /// Along both axis: dst[i, j] = src[src.rows - i - 1, src.cols - j - 1]
    XYAxis,
}

impl Mat {
    #[inline]
    /// Creates a `Mat` object from raw `CMat` pointer. This will read the rows
    /// and cols of the image.
    pub fn from_raw(raw: *mut CMat) -> Mat {
        Mat {
            inner: raw,
            rows: unsafe { cv_mat_rows(raw) },
            cols: unsafe { cv_mat_cols(raw) },
            depth: unsafe { cv_mat_depth(raw) },
        }
    }

    /// Creates an empty `Mat` struct.
    pub fn new() -> Mat {
        let m = unsafe { cv_mat_new() };
        Mat::from_raw(m)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn with_size(rows: i32, cols: i32, t: i32) -> Self {
        let m = unsafe { cv_mat_new_with_size(rows, cols, t) };
        Mat::from_raw(m)
    }

    /// Create a `Mat` from reading the image specified by the path.
    pub fn from_path(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { cv_imread((&s).as_ptr(), flags) };
        Mat::from_raw(m)
    }

    /// Returns the raw data (as a uchar pointer)
    pub fn data(&self) -> *const u8 {
        unsafe { cv_mat_data(self.inner) }
    }

    /// Returns the total number of array elements. The method returns the
    /// number of array elements (a number of pixels if the array represents an
    /// image). For example, images with 1920x1080 resolution will return
    /// 2073600.
    pub fn total(&self) -> usize {
        unsafe { cv_mat_total(self.inner) }
    }

    /// Returns the matrix element size in bytes. The method returns the matrix
    /// element size in bytes. For example, if the matrix type is CV_16SC3 , the
    /// method returns 3*sizeof(short) or 6.
    pub fn elem_size(&self) -> usize {
        unsafe { cv_mat_elem_size(self.inner) }
    }

    /// Returns the size of this matrix.
    pub fn size(&self) -> Size2i {
        Size2i::new(self.cols, self.rows)
    }

    /// Check if the `Mat` is valid or not.
    pub fn is_valid(&self) -> bool {
        unsafe { cv_mat_is_valid(self.inner) }
    }

    /// Return a region of interest from a `Mat` specfied by a `Rect`.
    pub fn roi(&self, rect: Rect) -> Mat {
        let cmat = unsafe { cv_mat_roi(self.inner, rect) };
        Mat::from_raw(cmat)
    }

    /// Apply a mask to myself.
    // TODO(benzh): Find the right reference in OpenCV for this one. Provide a
    // shortcut for `image &= mask`
    pub fn logic_and(&mut self, mask: Mat) {
        unsafe {
            cv_mat_logic_and(self.inner, mask.inner);
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
            cv_mat_flip(self.inner, code);
        }
    }

    /// Calls out to highgui to show the image, the duration is specified by
    /// `delay`.
    pub fn show(&self, name: &str, delay: i32) {
        extern "C" {
            fn cv_imshow(name: *const c_char, cmat: *mut CMat);
            fn cv_wait_key(delay_ms: c_int) -> c_int;
        }

        let s = CString::new(name).unwrap();
        unsafe {
            cv_imshow((&s).as_ptr(), self.inner);
            cv_wait_key(delay);
        }
    }

    /// Returns the images type. For supported types, please see
    /// [CvType](enum.CvType).
    pub fn cv_type(&self) -> CvType {
        num::FromPrimitive::from_i32(unsafe { cv_mat_type(self.inner) }).unwrap()
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            cv_mat_drop(self.inner);
        }
    }
}

/// Here is the `CvType` in an easy-to-read table.
///
/// |        | C1 | C2 | C3 | C4 | C(5) | C(6) | C(7) | C(8) |
/// |--------|----|----|----|----|------|------|------|------|
/// | CV_8U  |  0 |  8 | 16 | 24 |   32 |   40 |   48 |   56 |
/// | CV_8S  |  1 |  9 | 17 | 25 |   33 |   41 |   49 |   57 |
/// | CV_16U |  2 | 10 | 18 | 26 |   34 |   42 |   50 |   58 |
/// | CV_16S |  3 | 11 | 19 | 27 |   35 |   43 |   51 |   59 |
/// | CV_32S |  4 | 12 | 20 | 28 |   36 |   44 |   52 |   60 |
/// | CV_32F |  5 | 13 | 21 | 29 |   37 |   45 |   53 |   61 |
/// | CV_64F |  6 | 14 | 22 | 30 |   38 |   46 |   54 |   62 |
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum CvType {
    /// 8 bit, single channel (grey image)
    Cv8UC1 = 0,

    /// 8 bit, two channel (rarelly seen)
    Cv8UC2 = 8,

    /// 8 bit, three channels (RGB image)
    Cv8UC3 = 16,
}

/// This struct represents a rotated (i.e. not up-right) rectangle. Each
/// rectangle is specified by the center point (mass center), length of each
/// side (represented by `Size2f`) and the rotation angle in degrees.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct RotatedRect {
    center: Point2f,
    size: Size2f,
    angle: f32,
}

impl RotatedRect {
    /// Return 4 vertices of the rectangle.
    pub fn points(&self) -> [Point2f; 4] {
        let angle = self.angle * ::std::f32::consts::PI / 180.0;

        let b = angle.cos() * 0.5;
        let a = angle.sin() * 0.5;

        let mut pts: [Point2f; 4] = [Point2f::default(); 4];
        pts[0].x = self.center.x - a * self.size.height - b * self.size.width;
        pts[0].y = self.center.y + b * self.size.height - a * self.size.width;
        pts[1].x = self.center.x + a * self.size.height - b * self.size.width;
        pts[1].y = self.center.y - b * self.size.height - a * self.size.width;

        pts[2].x = 2.0 * self.center.x - pts[0].x;
        pts[2].y = 2.0 * self.center.y - pts[0].y;
        pts[3].x = 2.0 * self.center.x - pts[1].x;
        pts[3].y = 2.0 * self.center.y - pts[1].y;
        pts
    }

    /// Return the minimal up-right rectangle containing the rotated rectangle
    pub fn bounding_rect(&self) -> Rect {
        let pt = self.points();
        let x = pt.iter().map(|p| p.x).fold(0. / 0., f32::min).floor() as i32;
        let y = pt.iter().map(|p| p.y).fold(0. / 0., f32::min).floor() as i32;

        let width = pt.iter().map(|p| p.x).fold(0. / 0., f32::max).ceil() as i32 - x + 1;
        let height = pt.iter().map(|p| p.y).fold(0. / 0., f32::max).ceil() as i32 - y + 1;
        Rect::new(x, y, width, height)
    }
}

// =============================================================================
// core array
// =============================================================================
extern "C" {
    fn cv_in_range(cmat: *const CMat, lowerb: Scalar, upperb: Scalar, dst: *mut CMat);
    fn cv_mix_channels(cmat: *const CMat,
                       nsrcs: isize,
                       dst: *mut CMat,
                       ndsts: isize,
                       from_to: *const i32,
                       npairs: isize);
    fn cv_normalize(csrc: *const CMat, cdst: *mut CMat, alpha: c_double, beta: c_double, norm_type: c_int);
}

/// Normalization type. Please refer to [OpenCV's
/// documentation](http://docs.cv.org/trunk/d2/de8/group__core__array.html#gad12cefbcb5291cf958a85b4b67b6149f).
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
        unsafe { cv_in_range(self.inner, lowerb, upperb, m) }
        Mat::from_raw(m)
    }

    /// Copy specified channels from `self` to the specified channels of output
    /// `Mat`.
    // TODO(benzh) Avoid using raw pointers but rather take a vec for `from_to`?
    pub fn mix_channels(&self, nsrcs: isize, ndsts: isize, from_to: *const i32, npairs: isize) -> Mat {
        let m = Mat::with_size(self.rows, self.cols, self.depth);
        unsafe {
            cv_mix_channels(self.inner, nsrcs, m.inner, ndsts, from_to, npairs);
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormTypes) -> Mat {
        let m = CMat::new();
        unsafe { cv_normalize(self.inner, m, alpha, beta, t as i32) }
        Mat::from_raw(m)
    }
}
