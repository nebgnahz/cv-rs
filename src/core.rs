//! Core data structures in OpenCV

use bytes::{self, ByteOrder};
use errors::*;
use failure::Error;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_double, c_int, c_uchar};
use std::path::Path;
use std::slice;
use std::ops::BitAnd;

/// Opaque data struct for C bindings
#[derive(Clone, Copy, Debug)]
pub enum CMat {}
unsafe impl Send for CMat {}

impl CMat {
    pub(crate) fn new() -> *mut CMat {
        unsafe { cv_mat_new() }
    }
}

/// This wraps OpenCV's `Mat` class which is designed for n-dimensional dense
/// array. It's the most widely used data structure in image/video processing
/// since images are often stored as `Mat`.
#[derive(Debug)]
pub struct Mat {
    /// Pointer to the actual C/C++ data structure
    pub(crate) inner: *mut CMat,

    /// Number of columns
    pub cols: c_int,

    /// Number of rows
    pub rows: c_int,

    /// Depth of this mat (it should be the type).
    pub depth: c_int,

    /// Channels of this mat
    pub channels: c_int,
}

// TODO(benzh): Should consider Unique<T>,
// https://github.com/rust-lang/rust/issues/27730
unsafe impl Send for Mat {}

/// Data structure for salient point detectors
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct KeyPoint {
    /// Coordinates of the keypoint
    pub point: Point2f,
    /// Diameter of the meaningful keypoint neighborhood
    pub size: f32,
    /// Computed orientation of the keypoint (-1 if not applicable); it's in [0,360) degrees and measured relative to image coordinate system, ie in clockwise.
    pub angle: f32,
    /// The response by which the most strong keypoints have been selected. Can be used for the further sorting or subsampling
    pub response: f32,
    /// Octave (pyramid layer) from which the keypoint has been extracted
    pub octave: c_int,
    /// Object class (if the keypoints need to be clustered by an object they belong to)
    pub class_id: c_int,
}

/// A 4-element struct that is widely used to pass pixel values.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Scalar {
    v0: c_int,
    v1: c_int,
    v2: c_int,
    v3: c_int,
}

impl Scalar {
    /// Creates a new scalar object.
    pub fn new(v0: c_int, v1: c_int, v2: c_int, v3: c_int) -> Self {
        Scalar {
            v0: v0,
            v1: v1,
            v2: v2,
            v3: v3,
        }
    }

    /// Creates a new scalar object with all value being the same.
    pub fn all(v: c_int) -> Self {
        Scalar {
            v0: v,
            v1: v,
            v2: v,
            v3: v,
        }
    }
}

/// 2D integer points specified by its coordinates `x` and `y`.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2i {
    /// x coordinate
    pub x: c_int,

    /// y coordinate
    pub y: c_int,
}

impl Point2i {
    /// Creats a new `Point2i`.
    pub fn new(x: c_int, y: c_int) -> Self {
        Point2i { x: x, y: y }
    }
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

impl Point2f {
    /// Creats a new `Point2f`.
    pub fn new(x: f32, y: f32) -> Self {
        Point2f { x: x, y: y }
    }
}

/// `Size2i` struct is used for specifying the size of an image or rectangle with integer dimensions.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2i {
    /// width
    pub width: c_int,

    /// height
    pub height: c_int,
}

impl Size2i {
    /// Creates a new `Size2i` object with `width` and `height`
    pub fn new(width: c_int, height: c_int) -> Self {
        Size2i {
            width: width,
            height: height,
        }
    }
}

/// `Size2f` struct is used for specifying the size of an image or rectangle with float dimensions.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2f {
    /// width
    pub width: f32,

    /// height
    pub height: f32,
}

/// The `Rect` defines a rectangle in integer.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Rect {
    /// x coordinate of the left-top corner
    pub x: c_int,
    /// y coordinate of the left-top corner
    pub y: c_int,
    /// width of this rectangle
    pub width: c_int,
    /// height of this rectangle
    pub height: c_int,
}

impl Rect {
    /// Creates a new `Rect` with (x, y, width, height) parameters.
    pub fn new(x: c_int, y: c_int, width: c_int, height: c_int) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// Scales the rectangle by the specified ratio.
    pub fn scale(&self, ratio: f32) -> Rect {
        let new_x = ((1.0 - ratio) * (self.width as f32) / 2.0) as c_int + self.x;
        let new_y = ((1.0 - ratio) * (self.height as f32) / 2.0) as c_int + self.y;
        let new_w = ((self.width as f32) * ratio) as c_int;
        let new_h = ((self.height as f32) * ratio) as c_int;
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
            x: (self.x * mat.cols as f32) as c_int,
            y: (self.y * mat.rows as f32) as c_int,
            width: (self.width * mat.cols as f32) as c_int,
            height: (self.height * mat.rows as f32) as c_int,
        }
    }
}

/// Line type
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LineType {
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
    fn cv_from_file_storage(path: *const c_char, section: *const c_char) -> *mut CMat;
    fn cv_mat_new_with_size(rows: c_int, cols: c_int, t: c_int) -> *mut CMat;
    fn cv_mat_zeros(rows: c_int, cols: c_int, t: c_int) -> *mut CMat;
    fn cv_mat_from_buffer(rows: c_int, cols: c_int, t: c_int, buffer: *const c_uchar) -> *mut CMat;
    fn cv_mat_is_valid(mat: *mut CMat) -> bool;
    fn cv_mat_rows(cmat: *const CMat) -> c_int;
    fn cv_mat_cols(cmat: *const CMat) -> c_int;
    fn cv_mat_depth(cmat: *const CMat) -> c_int;
    fn cv_mat_channels(cmat: *const CMat) -> c_int;
    fn cv_mat_data(cmat: *const CMat) -> *const u8;
    fn cv_mat_total(cmat: *const CMat) -> usize;
    fn cv_mat_step1(cmat: *const CMat, i: c_int) -> usize;
    fn cv_mat_elem_size(cmat: *const CMat) -> usize;
    fn cv_mat_elem_size1(cmat: *const CMat) -> usize;
    fn cv_mat_type(cmat: *const CMat) -> CvType;
    fn cv_mat_roi(cmat: *const CMat, rect: Rect) -> *mut CMat;
    fn cv_mat_logic_and(cimage: *const CMat, cmask: *const CMat) -> *mut CMat;
    fn cv_mat_flip(src: *mut CMat, code: c_int);
    fn cv_mat_drop(mat: *mut CMat);
    fn cv_mat_eye(rows: c_int, cols: c_int, cv_type: CvType) -> *mut CMat;
}

/// A flag to specify how to flip the image. see
/// [Mat::flip](struct.Mat.html#method.flip)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FlipCode {
    /// Along x-axis: dst[i, j] = src[src.rows - i - 1, j]
    XAxis,
    /// Along y-axis: dst[i, j] = src[i, src.cols - j - 1]
    YAxis,
    /// Along both axis: dst[i, j] = src[src.rows - i - 1, src.cols - j - 1]
    XYAxis,
}

impl Mat {
    /// Loads `Mat` from file storage
    pub fn from_file_storage<P: AsRef<Path>>(path: P, section: &str) -> Result<Mat, Error> {
        let path = path_to_cstring(path)?;
        let section = CString::new(section)?;

        let path = path.as_ptr();
        let section = section.as_ptr();
        let result = unsafe { cv_from_file_storage(path, section) };
        Ok(Mat::from_raw(result))
    }

    #[inline]
    /// Creates a `Mat` object from raw `CMat` pointer. This will read the rows
    /// and cols of the image.
    pub(crate) fn from_raw(raw: *mut CMat) -> Mat {
        Mat {
            inner: raw,
            rows: unsafe { cv_mat_rows(raw) },
            cols: unsafe { cv_mat_cols(raw) },
            depth: unsafe { cv_mat_depth(raw) },
            channels: unsafe { cv_mat_channels(raw) },
        }
    }

    /// Creates an empty `Mat` struct.
    pub fn new() -> Mat {
        let m = CMat::new();
        Mat::from_raw(m)
    }

    /// Creates a new `Mat` from buffer. Note that internally opencv function
    /// won't take ownership of the Mat, but when we call `drop`, it will
    /// deallocate the memory. To prevent double-freeing, you must `mem::forget`
    /// it after use.
    ///
    /// The following example shows how to get the data from an image and create
    /// a new image with the data (also forgets it).
    ///
    /// ```rust,ignore
    /// let buffer = image.data();
    /// let size = image.size();
    /// let s = (size.width * size.height * 3) as usize;
    ///
    /// let mut vec = Vec::with_capacity(s);
    /// unsafe {
    ///   vec.set_len(s);
    ///   copy(buffer, vec.as_mut_ptr(), s);
    /// }
    /// let new_image = Mat::from_buffer(
    ///   size.height, size.width, CvType::Cv8UC3 as i32, &vec);
    ///
    ///  // . . . use new_image here, such as new_image.show(..) . . .
    ///
    /// ::std::mem::forget(new_image);
    /// ```
    pub fn from_buffer(rows: c_int, cols: c_int, cv_type: c_int, buf: &Vec<u8>) -> Mat {
        let raw = unsafe { cv_mat_from_buffer(rows, cols, cv_type, buf.as_ptr()) };
        Mat::from_raw(raw)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn with_size(rows: c_int, cols: c_int, t: c_int) -> Self {
        let m = unsafe { cv_mat_new_with_size(rows, cols, t) };
        Mat::from_raw(m)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn zeros(rows: c_int, cols: c_int, t: c_int) -> Self {
        let m = unsafe { cv_mat_zeros(rows, cols, t) };
        Mat::from_raw(m)
    }

    /// Returns the raw data (as a uchar pointer)
    pub fn data(&self) -> &[u8] {
        let bytes = unsafe { cv_mat_data(self.inner) };
        let len = self.total() * self.elem_size();
        unsafe { slice::from_raw_parts(bytes, len) }
    }

    /// Returns the total number of array elements. The method returns the
    /// number of array elements (a number of pixels if the array represents an
    /// image). For example, images with 1920x1080 resolution will return 2073600.
    pub fn total(&self) -> usize {
        unsafe { cv_mat_total(self.inner) }
    }

    /// Returns the matrix element size in bytes.
    ///
    /// The method returns the matrix element size in bytes. For example, if the
    /// matrix type is CV_16SC3 , the method returns 3*sizeof(short) or 6.
    pub fn elem_size(&self) -> usize {
        unsafe { cv_mat_elem_size(self.inner) }
    }

    /// Returns the size of each matrix element channel in bytes.
    ///
    /// The method returns the matrix element channel size in bytes, that
    /// is, it ignores the number of channels. For example, if the matrix
    /// type is CV_16SC3 , the method returns sizeof(short) or 2.
    pub fn elem_size1(&self) -> usize {
        unsafe { cv_mat_elem_size1(self.inner) }
    }

    /// Returns a normalized step.
    ///
    /// The method returns a matrix step divided by Mat::elemSize1() . It can be
    /// useful to quickly access an arbitrary matrix element
    pub fn step1(&self, i: c_int) -> usize {
        unsafe { cv_mat_step1(self.inner, i) }
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
    pub fn show(&self, name: &str, delay: c_int) -> Result<(), Error> {
        extern "C" {
            fn cv_imshow(name: *const c_char, cmat: *mut CMat);
            fn cv_wait_key(delay_ms: c_int) -> c_int;
        }

        let s = CString::new(name)?;
        unsafe {
            cv_imshow((&s).as_ptr(), self.inner);
            cv_wait_key(delay);
        }
        Ok(())
    }

    /// Returns the images type. For supported types, please see
    /// [CvType](enum.CvType).
    pub fn cv_type(&self) -> CvType {
        unsafe { cv_mat_type(self.inner) }
    }

    /// Returns an identity matrix of the specified size and type.
    pub fn eye(rows: i32, cols: i32, cv_type: CvType) -> Mat {
        let result = unsafe { cv_mat_eye(rows, cols, cv_type) };
        Mat::from_raw(result)
    }
}

impl BitAnd for Mat {
    type Output = Self;

    /// Apply a mask to image. See [OpenCV reference](https://docs.opencv.org/2.4/modules/core/doc/operations_on_arrays.html#bitwise-and)
    fn bitand(self, rhs: Self) -> Self::Output {
        let result = unsafe { cv_mat_logic_and(self.inner, rhs.inner) };

        Self::from_raw(result)
    }
}

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl<T: FromBytes> FromBytes for (T, T, T) {
    fn from_bytes(bytes: &[u8]) -> (T, T, T) {
        let size = mem::size_of::<T>();
        (
            T::from_bytes(&bytes[(0 * size)..(1 * size)]),
            T::from_bytes(&bytes[(1 * size)..(2 * size)]),
            T::from_bytes(&bytes[(2 * size)..(3 * size)]),
        )
    }
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> u8 {
        bytes[0]
    }
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> i8 {
        bytes[0] as i8
    }
}

impl FromBytes for u16 {
    #[cfg(target_endian = "big")]
    fn from_bytes(bytes: &[u8]) -> u16 {
        bytes::BigEndian::read_u16(bytes)
    }

    #[cfg(target_endian = "little")]
    fn from_bytes(bytes: &[u8]) -> u16 {
        bytes::LittleEndian::read_u16(bytes)
    }
}

impl FromBytes for i16 {
    #[cfg(target_endian = "big")]
    fn from_bytes(bytes: &[u8]) -> i16 {
        bytes::BigEndian::read_i16(bytes)
    }

    #[cfg(target_endian = "little")]
    fn from_bytes(bytes: &[u8]) -> i16 {
        bytes::LittleEndian::read_i16(bytes)
    }
}

impl FromBytes for f32 {
    #[cfg(target_endian = "big")]
    fn from_bytes(bytes: &[u8]) -> f32 {
        bytes::BigEndian::read_f32(bytes)
    }

    #[cfg(target_endian = "little")]
    fn from_bytes(bytes: &[u8]) -> f32 {
        bytes::LittleEndian::read_f32(bytes)
    }
}

impl FromBytes for i32 {
    #[cfg(target_endian = "big")]
    fn from_bytes(bytes: &[u8]) -> i32 {
        bytes::BigEndian::read_i32(bytes)
    }

    #[cfg(target_endian = "little")]
    fn from_bytes(bytes: &[u8]) -> i32 {
        bytes::LittleEndian::read_i32(bytes)
    }
}

impl FromBytes for f64 {
    #[cfg(target_endian = "big")]
    fn from_bytes(bytes: &[u8]) -> f64 {
        bytes::BigEndian::read_f64(bytes)
    }

    #[cfg(target_endian = "little")]
    fn from_bytes(bytes: &[u8]) -> f64 {
        bytes::LittleEndian::read_f64(bytes)
    }
}

impl Mat {
    /// Returns individual pixel (element) information within the Mat. This
    /// function may need type annotation to assist `FromBytes` trait.
    ///
    /// - If matrix is of type `CV_8U` then use `Mat.at<u8>(y,x)`.
    /// - If matrix is of type `CV_8S` then use `Mat.at<i8>(y,x)`.
    /// - If matrix is of type `CV_16U` then use `Mat.at<u16>(y,x)`.
    /// - If matrix is of type `CV_16S` then use `Mat.at<i16>(y,x)`.
    /// - If matrix is of type `CV_32S`  then use `Mat.at<i32>(y,x)`.
    /// - If matrix is of type `CV_32F`  then use `Mat.at<f32>(y,x)`.
    /// - If matrix is of type `CV_64F` then use `Mat.at<f64>(y,x)`.
    pub fn at<T: FromBytes>(&self, i0: i32) -> T {
        let data = self.data();
        let size = self.size();
        let pos = {
            if size.height == 1 {
                i0 as usize
            } else if size.width == 1 {
                i0 as usize * (self.step1(1) * self.elem_size1())
            } else {
                unimplemented!{};
            }
        };

        let byte = &data[pos];
        let ptr: *const _ = byte;
        let slice = unsafe { slice::from_raw_parts(ptr, mem::size_of::<T>()) };
        T::from_bytes(slice)
    }

    /// Returns individual pixel (element) information within the Mat. This
    /// function may need type annotation to assist `FromBytes` trait.
    ///
    /// See [Mat::at](struct.Mat.html#method.at) and
    /// [Mat::at3](struct.Mat.html#method.at3).
    pub fn at2<T: FromBytes>(&self, i0: i32, i1: i32) -> T {
        let data = self.data();
        let pos = i0 as usize * self.step1(0) * self.elem_size1() + i1 as usize * self.step1(1) * self.elem_size1();
        let byte = &data[pos];
        let ptr: *const _ = byte;
        let slice = unsafe { slice::from_raw_parts(ptr, mem::size_of::<T>()) };
        T::from_bytes(slice)
    }

    /// Returns individual pixel (element) information within the Mat. This
    /// function may need type annotation to assist `FromBytes` trait.
    ///
    /// See [Mat::at](struct.Mat.html#method.at) and
    /// [Mat::at2](struct.Mat.html#method.at2).
    pub fn at3<T: FromBytes>(&self, i0: i32, i1: i32, i2: i32) -> T {
        let data = self.data();
        let pos = i0 as usize * self.step1(0) * self.elem_size1() + i1 as usize * self.step1(1) * self.elem_size1()
            + i2 as usize;
        let byte = &data[pos];
        let ptr: *const _ = byte;
        let slice = unsafe { slice::from_raw_parts(ptr, mem::size_of::<T>()) };
        T::from_bytes(slice)
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
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CvType {
    /// 8 bit unsigned (like `uchar`), single channel (grey image)
    Cv8UC1 = 0,

    /// 8 bit signed (like `schar`), single channel (grey image)
    Cv8SC1 = 1,

    /// 16 bit unsigned (like `ushort`), single channel (grey image)
    Cv16UC1 = 2,

    /// 16 bit signed (like `short`), single channel (grey image)
    Cv16SC1 = 3,

    /// 32 bit signed (like `int`), single channel (grey image)
    Cv32SC1 = 4,

    /// 32 bit float (like `float`), single channel (grey image)
    Cv32FC1 = 5,

    /// 32 bit float (like `double`), single channel (grey image)
    Cv64FC1 = 6,

    /// 8 bit, two channel (rarelly seen)
    Cv8UC2 = 8,

    /// 8 bit unsigned (like `uchar`), three channels (RGB image)
    Cv8UC3 = 16,

    /// 8 bit signed (like `schar`), three channels (RGB image)
    Cv8SC3 = 17,

    /// 16 bit unsigned (like `ushort`), three channels (RGB image)
    Cv16UC3 = 18,

    /// 16 bit signed (like `short`), three channels (RGB image)
    Cv16SC3 = 19,

    /// 32 bit signed (like `int`), three channels (RGB image)
    Cv32SC3 = 20,

    /// 32 bit float (like `float`), three channels (RGB image)
    Cv32FC3 = 21,

    /// 32 bit float (like `double`), three channels (RGB image)
    Cv64FC3 = 22,
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
        let x = pt.iter().map(|p| p.x).fold(0. / 0., f32::min).floor() as c_int;
        let y = pt.iter().map(|p| p.y).fold(0. / 0., f32::min).floor() as c_int;

        let width = pt.iter().map(|p| p.x).fold(0. / 0., f32::max).ceil() as c_int - x + 1;
        let height = pt.iter().map(|p| p.y).fold(0. / 0., f32::max).ceil() as c_int - y + 1;
        Rect::new(x, y, width, height)
    }
}

// =============================================================================
// core array
// =============================================================================
extern "C" {
    fn cv_in_range(cmat: *const CMat, lowerb: Scalar, upperb: Scalar, dst: *mut CMat);
    fn cv_min_max_loc(
        cmat: *const CMat,
        min: *mut f64,
        max: *mut f64,
        min_loc: *mut Point2i,
        max_loc: *mut Point2i,
        cmask: *const CMat,
    );
    fn cv_mix_channels(
        cmat: *const CMat,
        nsrcs: usize,
        dst: *mut CMat,
        ndsts: usize,
        from_to: *const c_int,
        npairs: usize,
    );
    fn cv_normalize(csrc: *const CMat, cdst: *mut CMat, alpha: c_double, beta: c_double, norm_type: NormType);

    fn cv_bitwise_and(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    fn cv_bitwise_not(src: *const CMat, dst: *mut CMat);
    fn cv_bitwise_or(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    fn cv_bitwise_xor(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    fn cv_count_non_zero(src: *const CMat) -> c_int;
}

/// Normalization type. Please refer to [OpenCV's
/// documentation](http://docs.cv.org/trunk/d2/de8/group__core__array.html).
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum NormType {
    /// Normalized using `max`
    Inf = 1,
    /// Normalized using L1 distance
    L1 = 2,
    /// Normalized using L2 distance
    L2 = 4,
    /// Normalized using L2 sqr distance
    L2Sqr = 5,
    /// Normalized using hamming distance
    Hamming = 6,
    /// Normalized using hamming2 distance
    Hamming2 = 7,
    /// Normalized using relative distance
    Relative = 8,
    /// Normalized using minmax distance
    MinMax = 32,
}

impl Mat {
    /// Checks if Mat elements lie between the elements of two other arrays
    /// (lowerb and upperb). The output Mat has the same size as `self` and
    /// CV_8U type.
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = CMat::new();
        unsafe { cv_in_range(self.inner, lowerb, upperb, m) }
        Mat::from_raw(m)
    }

    /// Finds the global minimum and maximum in an array.
    ///
    /// This function finds the minimum and maximum element values and their
    /// positions. The extremums are searched across the whole array or, if mask
    /// is not an empty array, in the specified array region.
    ///
    /// N.B. Only work with single-channel Mat. For multi-channel arrays. If you
    /// need to find minimum or maximum elements across all the channels, use
    /// Mat::reshape first to reinterpret the array as single-channel. Or you
    /// may extract the particular channel using either extractImageCOI , or
    /// mixChannels, or split.
    pub fn min_max_loc(&self, mask: Mat) -> (f64, f64, Point2i, Point2i) {
        let mut min = 0.0;
        let mut max = 0.0;
        let mut min_loc = Point2i::new(0, 0);
        let mut max_loc = Point2i::new(0, 0);
        unsafe {
            cv_min_max_loc(
                self.inner,
                &mut min,
                &mut max,
                &mut min_loc,
                &mut max_loc,
                mask.inner,
            )
        }
        (min, max, min_loc, max_loc)
    }

    /// Copy specified channels from `self` to the specified channels of output
    /// `Mat`.
    // The usage (self.depth) here is buggy, it should actually be the type!
    pub fn mix_channels<T: AsRef<[(c_int, c_int)]>>(&self, nsrcs: usize, ndsts: usize, from_to: T) -> Mat {
        let m = Mat::with_size(self.rows, self.cols, self.depth);
        let slice = from_to.as_ref();
        let ptr = slice.as_ptr() as *const c_int;
        unsafe {
            cv_mix_channels(self.inner, nsrcs, m.inner, ndsts, ptr, slice.len());
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormType) -> Mat {
        let m = CMat::new();
        unsafe { cv_normalize(self.inner, m, alpha, beta, t) }
        Mat::from_raw(m)
    }

    /// Computes bitwise conjunction between two Mat
    pub fn and(&self, another: &Mat) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_and(self.inner, another.inner, m) }
        Mat::from_raw(m)
    }

    /// Computes bitwise disjunction between two Mat
    pub fn or(&self, another: &Mat) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_or(self.inner, another.inner, m) }
        Mat::from_raw(m)
    }

    /// Computes bitwise "exclusive or" between two Mat
    pub fn xor(&self, another: &Mat) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_xor(self.inner, another.inner, m) }
        Mat::from_raw(m)
    }

    /// Computes bitwise "exclusive or" between two Mat
    pub fn not(&self) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_not(self.inner, m) }
        Mat::from_raw(m)
    }

    /// Counts non-zero array elements.
    pub fn count_non_zero(&self) -> c_int {
        unsafe { cv_count_non_zero(self.inner) }
    }
}

pub(crate) fn path_to_cstring<P: AsRef<Path>>(path: P) -> Result<CString, Error> {
    let path = path.as_ref();
    let x = path.to_str().ok_or(CvError::InvalidPath(path.into()))?;
    let result = CString::new(x)?;
    Ok(result)
}
