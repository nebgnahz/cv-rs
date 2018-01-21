//! Core data structures in OpenCV

use bytes::{self, ByteOrder};
use errors::*;
use std::os::raw::{c_char, c_double, c_int, c_uchar, c_void};
use num;
use std::ffi::CString;
use std::mem;
use std::slice;

/// Opaque data struct for C bindings
#[derive(Clone, Copy, Debug)]
pub enum CMat {}
unsafe impl Send for CMat {}
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

    /// Depth of this mat (it should be the type).
    pub depth: i32,

    /// Channels of this mat
    pub channels: i32,
}

// TODO(benzh): Should consider Unique<T>,
// https://github.com/rust-lang/rust/issues/27730
unsafe impl Send for Mat {}

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

    /// Creates a new scalar object with all value being the same.
    pub fn all(v: i32) -> Self {
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
    pub x: i32,

    /// y coordinate
    pub y: i32,
}

impl Point2i {
    /// Creats a new `Point2i`.
    pub fn new(x: i32, y: i32) -> Self {
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
#[derive(Debug, Clone)]
pub struct CVec<T: Sized + NestedVec> {
    array: *mut T,
    size: usize,
}

// Unsafe because CVec is not guaranteed to contain valid pointer and size
unsafe fn unpack<T: NestedVec, U, F>(v: &CVec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    (0..v.size)
        .map(|i| f(&*v.array.offset(i as isize)))
        .collect()
}

pub trait Unpack {
    type Out;
    fn unpack(&self) -> Self::Out;
}

impl<T: Unpack + NestedVec> Unpack for CVec<T> {
    type Out = Vec<T::Out>;
    fn unpack(&self) -> Self::Out {
        unsafe { unpack(self, |e| e.unpack()) }
    }
}

impl<T: Copy> Unpack for T {
    type Out = T;
    fn unpack(&self) -> Self::Out {
        *self
    }
}

pub trait NestedVec {
    const LEVEL: u32;
}

impl<T: NestedVec> NestedVec for CVec<T> {
    const LEVEL: u32 = T::LEVEL + 1;
}

impl<T: Copy> NestedVec for T {
    const LEVEL: u32 = 0;
}

impl<T: NestedVec> Default for CVec<T> {
    fn default() -> Self {
        CVec {
            array: ::std::ptr::null_mut::<T>(),
            size: 0,
        }
    }
}

impl<T: NestedVec> Drop for CVec<T> {
    fn drop(&mut self) {
        extern "C" {
            fn cv_vec_drop(vec: *mut c_void, depth: u32);
        }
        unsafe {
            let depth = CVec::<T>::LEVEL;
            let self_ptr: *mut _ = self;
            let self_ptr: *mut c_void = self_ptr as *mut _;
            cv_vec_drop(self_ptr, depth);
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
    fn cv_mat_zeros(rows: c_int, cols: c_int, t: i32) -> *mut CMat;
    fn cv_mat_from_buffer(rows: c_int, cols: c_int, t: i32, buffer: *const c_uchar) -> *mut CMat;
    fn cv_mat_is_valid(mat: *mut CMat) -> bool;
    fn cv_mat_rows(cmat: *const CMat) -> c_int;
    fn cv_mat_cols(cmat: *const CMat) -> c_int;
    fn cv_mat_depth(cmat: *const CMat) -> c_int;
    fn cv_mat_channels(cmat: *const CMat) -> c_int;
    fn cv_mat_data(cmat: *const CMat) -> *const c_uchar;
    fn cv_mat_total(cmat: *const CMat) -> usize;
    fn cv_mat_step1(cmat: *const CMat, i: c_int) -> usize;
    fn cv_mat_elem_size(cmat: *const CMat) -> usize;
    fn cv_mat_elem_size1(cmat: *const CMat) -> usize;
    fn cv_mat_type(cmat: *const CMat) -> c_int;
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
            channels: unsafe { cv_mat_channels(raw) },
        }
    }

    /// Creates an empty `Mat` struct.
    pub fn new() -> Mat {
        let m = unsafe { cv_mat_new() };
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
    pub fn from_buffer(rows: i32, cols: i32, cv_type: i32, buf: &Vec<u8>) -> Mat {
        let raw = unsafe { cv_mat_from_buffer(rows, cols, cv_type, buf.as_ptr()) };
        Mat::from_raw(raw)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn with_size(rows: i32, cols: i32, t: i32) -> Self {
        let m = unsafe { cv_mat_new_with_size(rows, cols, t) };
        Mat::from_raw(m)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn zeros(rows: i32, cols: i32, t: i32) -> Self {
        let m = unsafe { cv_mat_zeros(rows, cols, t) };
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
    pub fn show(&self, name: &str, delay: i32) -> Result<()> {
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
    pub fn cv_type(&self) -> Result<CvType> {
        let t = unsafe { cv_mat_type(self.inner) };
        let e = ErrorKind::NumFromPrimitive(t as i64).into();
        num::FromPrimitive::from_i32(t).ok_or(e)
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
        let data: *const u8 = self.data();
        let size = self.size();
        let pos = {
            if size.height == 1 {
                i0
            } else if size.width == 1 {
                i0 * (self.step1(1) * self.elem_size1()) as i32
            } else {
                unimplemented!{};
            }
        } as isize;
        unsafe {
            let ptr: *const u8 = data.offset(pos);
            let slice = slice::from_raw_parts(ptr, mem::size_of::<T>());
            T::from_bytes(slice)
        }
    }

    /// Returns individual pixel (element) information within the Mat. This
    /// function may need type annotation to assist `FromBytes` trait.
    ///
    /// See [Mat::at](struct.Mat.html#method.at) and
    /// [Mat::at3](struct.Mat.html#method.at3).
    pub fn at2<T: FromBytes>(&self, i0: i32, i1: i32) -> T {
        let data: *const u8 = self.data();
        let pos = (i0 as isize) * ((self.step1(0) * self.elem_size1()) as isize)
            + (i1 as isize) * ((self.step1(1) * self.elem_size1()) as isize);
        unsafe {
            let ptr: *const u8 = data.offset(pos);
            let slice = slice::from_raw_parts(ptr, mem::size_of::<T>());
            T::from_bytes(slice)
        }
    }

    /// Returns individual pixel (element) information within the Mat. This
    /// function may need type annotation to assist `FromBytes` trait.
    ///
    /// See [Mat::at](struct.Mat.html#method.at) and
    /// [Mat::at2](struct.Mat.html#method.at2).
    pub fn at3<T: FromBytes>(&self, i0: i32, i1: i32, i2: i32) -> T {
        let data: *const u8 = self.data();
        let pos = (i0 as isize) * ((self.step1(0) * self.elem_size1()) as isize)
            + (i1 as isize) * ((self.step1(1) * self.elem_size1()) as isize) + i2 as isize;
        unsafe {
            let ptr: *const u8 = data.offset(pos);
            let slice = slice::from_raw_parts(ptr, mem::size_of::<T>());
            T::from_bytes(slice)
        }
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
        nsrcs: isize,
        dst: *mut CMat,
        ndsts: isize,
        from_to: *const i32,
        npairs: isize,
    );
    fn cv_normalize(csrc: *const CMat, cdst: *mut CMat, alpha: c_double, beta: c_double, norm_type: c_int);

    fn cv_bitwise_and(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    fn cv_bitwise_not(src: *const CMat, dst: *mut CMat);
    fn cv_bitwise_or(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    fn cv_bitwise_xor(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    fn cv_count_non_zero(src: *const CMat) -> i32;
}

/// Normalization type. Please refer to [OpenCV's
/// documentation](http://docs.cv.org/trunk/d2/de8/group__core__array.html).
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
    // TODO(benzh) Avoid using raw pointers but rather take a vec for `from_to`?
    // The usage (self.depth) here is buggy, it should actually be the type!
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
    pub fn count_non_zero(&self) -> i32 {
        unsafe { cv_count_non_zero(self.inner) }
    }
}
