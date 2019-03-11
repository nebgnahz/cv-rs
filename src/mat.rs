//! Mat

use core::*;
use failure::Error;
use std::ffi::CString;
use std::mem;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::os::raw::c_int;
use std::path::Path;
use std::slice;
use *;

/// The class `Mat` represents an n-dimensional dense numerical single-channel or multi-channel array.
/// It can be used to store real or complex-valued vectors and matrices, grayscale or color images,
/// voxel volumes, vector fields, point clouds, tensors, histograms
#[derive(Debug)]
pub struct Mat {
    /// Pointer to the actual C/C++ data structure
    pub(crate) inner: *mut native::cv_Mat,

    /// Number of columns
    pub cols: c_int,

    /// Number of rows
    pub rows: c_int,

    /// Depth of this mat (it should be the type).
    pub depth: c_int,

    /// Channels of this mat
    pub channels: c_int,
}

impl Mat {
    /// Loads `Mat` from file storage
    pub fn from_file_storage<P: AsRef<Path>>(path: P, section: &str) -> Result<Mat, Error> {
        let path = path_to_cstring(path)?;
        let section = CString::new(section)?;

        let path = path.as_ptr();
        let section = section.as_ptr();
        let result = unsafe { native::cvsys_mat_from_file_storage(path, section) };
        Ok(unsafe { Mat::from_raw(result) })
    }

    #[inline]
    /// Creates a `Mat` object from raw `CMat` pointer. This will read the rows
    /// and cols of the image.
    pub(crate) unsafe fn from_raw(raw: *mut native::cv_Mat) -> Mat {
        Mat {
            inner: raw,
            rows: native::cvsys_mat_rows(raw),
            cols: native::cvsys_mat_cols(raw),
            depth: native::cvsys_mat_depth(raw),
            channels: native::cvsys_mat_channels(raw),
        }
    }

    /// Creates an empty `Mat` struct.
    pub fn new() -> Mat {
        unsafe {
            let m = native::cvsys_mat_new();
            Mat::from_raw(m)
        }
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
    pub unsafe fn from_buffer(rows: c_int, cols: c_int, cvsys_type: CvType, buf: &[u8]) -> Mat {
        let raw = native::cvsys_mat_from_buffer(rows, cols, cvsys_type as i32, buf.as_ptr());
        Mat::from_raw(raw)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn with_size(rows: c_int, cols: c_int, t: c_int) -> Self {
        unsafe {
            let m = native::cvsys_mat_new_with_size(rows, cols, t);
            Mat::from_raw(m)
        }
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn zeros(rows: c_int, cols: c_int, t: c_int) -> Self {
        unsafe {
            let m = native::cvsys_mat_zeros(rows, cols, t);
            Mat::from_raw(m)
        }
    }

    /// Returns the raw data (as a `u8` pointer)
    pub fn data(&self) -> &[u8] {
        let bytes = unsafe { native::cvsys_mat_data(self.inner) };
        let len = self.total() * self.elem_size();
        unsafe { slice::from_raw_parts(bytes, len) }
    }

    /// Returns the total number of array elements. The method returns the
    /// number of array elements (a number of pixels if the array represents an
    /// image). For example, images with 1920x1080 resolution will return 2073600.
    pub fn total(&self) -> usize {
        unsafe { native::cvsys_mat_total(self.inner) }
    }

    /// Returns the matrix element size in bytes.
    ///
    /// The method returns the matrix element size in bytes. For example, if the
    /// matrix type is CV_16SC3 , the method returns 3*sizeof(short) or 6.
    pub fn elem_size(&self) -> usize {
        unsafe { native::cvsys_mat_elem_size(self.inner) }
    }

    /// Returns the size of each matrix element channel in bytes.
    ///
    /// The method returns the matrix element channel size in bytes, that
    /// is, it ignores the number of channels. For example, if the matrix
    /// type is CV_16SC3 , the method returns sizeof(short) or 2.
    pub fn elem_size1(&self) -> usize {
        unsafe { native::cvsys_mat_elem_size1(self.inner) }
    }

    /// Returns a normalized step.
    ///
    /// The method returns a matrix step divided by Mat::elemSize1() . It can be
    /// useful to quickly access an arbitrary matrix element
    pub fn step1(&self, i: c_int) -> usize {
        unsafe { native::cvsys_mat_step1(self.inner, i) }
    }

    /// Returns the size of this matrix.
    pub fn size(&self) -> Size2i {
        Size2i::new(self.cols, self.rows)
    }

    /// Check if the `Mat` is valid or not.
    pub fn is_valid(&self) -> bool {
        unsafe { native::cvsys_mat_valid(self.inner) }
    }

    /// Return a region of interest from a `Mat` specfied by a `Rect`.
    pub fn roi(&self, rect: Rect) -> Mat {
        unsafe {
            let cmat = native::cvsys_mat_roi(self.inner, rect.into());
            Mat::from_raw(cmat)
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
            native::cvsys_mat_flip(self.inner, code);
        }
    }

    /// Returns the images type. For supported types, please see
    /// [CvType](enum.CvType).
    pub fn cvsys_type(&self) -> CvType {
        unsafe { native::cvsys_mat_type(self.inner).into() }
    }

    /// Returns an identity matrix of the specified size and type.
    pub fn eye(rows: i32, cols: i32, cvsys_type: CvType) -> Mat {
        unsafe {
            let result = native::cvsys_mat_eye(rows, cols, cvsys_type as i32);
            Mat::from_raw(result)
        }
    }

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
                unimplemented! {};
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
        let pos = i0 as usize * self.step1(0) * self.elem_size1()
            + i1 as usize * self.step1(1) * self.elem_size1()
            + i2 as usize;
        let byte = &data[pos];
        let ptr: *const _ = byte;
        let slice = unsafe { slice::from_raw_parts(ptr, mem::size_of::<T>()) };
        T::from_bytes(slice)
    }

    /// Checks if Mat elements lie between the elements of two other arrays
    /// (lowerb and upperb). The output Mat has the same size as `self` and
    /// CV_8U type.
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = Mat::new();
        unsafe { native::cvsys_mat_in_range(self.inner, lowerb.into(), upperb.into(), m.inner) }
        m
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
    pub fn min_max_loc(&self, mask: &Mat) -> (f64, f64, Point2i, Point2i) {
        let mut min = 0.0;
        let mut max = 0.0;
        let mut min_loc = Point2i::new(0, 0).into();
        let mut max_loc = Point2i::new(0, 0).into();
        unsafe { native::cvsys_mat_min_max_loc(self.inner, &mut min, &mut max, &mut min_loc, &mut max_loc, mask.inner) }
        (min, max, min_loc.into(), max_loc.into())
    }

    /// Copy specified channels from `self` to the specified channels of output
    /// `Mat`.
    // The usage (self.depth) here is buggy, it should actually be the type!
    pub fn mix_channels<T: AsRef<[(c_int, c_int)]>>(&self, nsrcs: usize, ndsts: usize, from_to: T) -> Mat {
        let m = Mat::with_size(self.rows, self.cols, self.depth);
        let slice = from_to.as_ref();
        let ptr = slice.as_ptr() as *const c_int;
        unsafe {
            native::cvsys_mat_mix_channels(self.inner, nsrcs, m.inner, ndsts, ptr, slice.len());
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormType) -> Mat {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_normalize(self.inner, m, alpha, beta, t as i32);
            Mat::from_raw(m)
        }
    }

    /// Counts non-zero array elements.
    pub fn count_non_zero(&self) -> c_int {
        unsafe { native::cvsys_mat_count_non_zero(self.inner) }
    }

    /// Forms a border around an image.
    ///
    /// The function copies the source image into the middle of the destination
    /// image. The areas to the left, to the right, above and below the copied
    /// source image will be filled with extrapolated pixels. This is not what
    /// filtering functions based on it do (they extrapolate pixels on-fly), but
    /// what other more complex functions, including your own, may do to
    /// simplify image boundary handling.
    pub fn copy_make_border(
        &self,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
        type_: BorderType,
        color: Scalar,
    ) -> Mat {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_copy_make_border(self.inner, m, top, bottom, left, right, type_ as i32, color.into());
            Mat::from_raw(m)
        }
    }
}

/// Various border types, image boundaries are denoted with `|`.
#[derive(Debug, Copy, Clone)]
pub enum BorderType {
    /// `iiiiii|abcdefgh|iiiiiii`  with some specified `i`
    Constant = 0,
    /// `aaaaaa|abcdefgh|hhhhhhh`
    Replicate = 1,
    /// `fedcba|abcdefgh|hgfedcb`
    Reflect = 2,
    /// `cdefgh|abcdefgh|abcdefg`
    Wrap = 3,
    /// `gfedcb|abcdefgh|gfedcba`
    Reflect101 = 4,
    /// `uvwxyz|abcdefgh|ijklmno`
    Transparent = 5,
    /// Do not look outside of ROI.
    Isolated = 16,
}

impl BorderType {
    #[allow(non_upper_case_globals)]
    /// same as Reflect101
    pub const Default: BorderType = BorderType::Reflect101;
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            native::cvsys_mat_drop(self.inner);
        }
    }
}

impl BitAnd for Mat {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_and(self.inner, rhs.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl<'a> BitAnd for &'a Mat {
    type Output = Mat;
    fn bitand(self, rhs: &'a Mat) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_and(self.inner, rhs.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl BitOr for Mat {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_or(self.inner, rhs.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl<'a> BitOr for &'a Mat {
    type Output = Mat;
    fn bitor(self, rhs: &'a Mat) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_or(self.inner, rhs.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl BitXor for Mat {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_xor(self.inner, rhs.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl<'a> BitXor for &'a Mat {
    type Output = Mat;
    fn bitxor(self, rhs: &'a Mat) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_xor(self.inner, rhs.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl Not for Mat {
    type Output = Self;
    fn not(self) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_not(self.inner, m);
            Mat::from_raw(m)
        }
    }
}

impl Clone for Mat {
    fn clone(&self) -> Self {
        unsafe { Mat::from_buffer(self.rows, self.cols, self.cvsys_type(), self.data()) }
    }
}

impl<'a> Not for &'a Mat {
    type Output = Mat;
    fn not(self) -> Self::Output {
        unsafe {
            let m = native::cvsys_mat_new();
            native::cvsys_mat_bitwise_not(self.inner, m);
            Mat::from_raw(m)
        }
    }
}
