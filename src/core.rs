//! Core data structures in OpenCV

use bytes::{self, ByteOrder};
use mat::*;
use std::mem;
use std::os::raw::c_int;

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
    /// x coordinate of the left-top corner
    pub x: f32,
    /// y coordinate of the left-top corner
    pub y: f32,
    /// width of this rectangle
    pub width: f32,
    /// height of this rectangle
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

/// Allow self deserialization from byte slice
pub trait FromBytes {
    /// Deserializes self from byte slice
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl<T: FromBytes> FromBytes for (T, T, T) {
    #[allow(clippy::erasing_op, clippy::identity_op)]
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
    /// 8 bit unsigned, single channel (grey image)
    Cv8UC1 = 0,
    /// 8 bit signed, single channel (grey image)
    Cv8SC1 = 1,
    /// 16 bit unsigned, single channel (grey image)
    Cv16UC1 = 2,
    /// 16 bit signed, single channel (grey image)
    Cv16SC1 = 3,
    /// 32 bit signed, single channel (grey image)
    Cv32SC1 = 4,
    /// 32 bit float, single channel (grey image)
    Cv32FC1 = 5,
    /// 32 bit float, single channel (grey image)
    Cv64FC1 = 6,
    /// 8 bit, two channel (rarelly seen)
    Cv8UC2 = 8,
    /// 8 bit unsigned, three channels (RGB image)
    Cv8UC3 = 16,
    /// 8 bit signed, three channels (RGB image)
    Cv8SC3 = 17,
    /// 16 bit unsigned, three channels (RGB image)
    Cv16UC3 = 18,
    /// 16 bit signed, three channels (RGB image)
    Cv16SC3 = 19,
    /// 32 bit signed, three channels (RGB image)
    Cv32SC3 = 20,
    /// 32 bit float, three channels (RGB image)
    Cv32FC3 = 21,
    /// 32 bit float, three channels (RGB image)
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
        use std::f32::NAN;
        let pt = self.points();
        let x = pt.iter().map(|p| p.x).fold(NAN, f32::min).floor() as c_int;
        let y = pt.iter().map(|p| p.y).fold(NAN, f32::min).floor() as c_int;

        let width = pt.iter().map(|p| p.x).fold(NAN, f32::max).ceil() as c_int - x + 1;
        let height = pt.iter().map(|p| p.y).fold(NAN, f32::max).ceil() as c_int - y + 1;
        Rect::new(x, y, width, height)
    }
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

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Term criteria type, can be one of: Count, Eps or Count + Eps
pub enum TermType {
    /// The maximum number of iterations or elements to compute
    Count = 1,

    /// the desired accuracy or change in parameters at which the iterative
    /// algorithm stops.
    EPS = 2,
}

/// Termination criteria for iterative algorithms.
#[derive(Debug)]
pub struct TermCriteria {
    pub(crate) c_criteria: *mut CTermCriteria,
}

impl TermCriteria {
    /// Creates a new termination criteria.
    pub fn new(t: TermType, max_count: c_int, epsilon: f64) -> Self {
        let c_criteria = unsafe { cv_term_criteria_new(t, max_count, epsilon) };
        TermCriteria { c_criteria: c_criteria }
    }
}

impl Drop for TermCriteria {
    fn drop(&mut self) {
        unsafe {
            cv_term_criteria_drop(self.c_criteria);
        }
    }
}
