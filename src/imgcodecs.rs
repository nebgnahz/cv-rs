//! Image file reading and writing, see [OpenCV
//! imgcodecs](http://docs.opencv.org/3.1.0/d4/da8/group__imgcodecs.html).

use errors::*;
use failure::Error;
use mat::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::path::Path;
use *;

extern "C" {
    fn cv_imread(input: *const c_char, flags: ImageReadMode) -> *mut CMat;
    fn cv_imdecode(buf: *const u8, l: usize, m: ImageReadMode) -> *mut CMat;
    fn cv_imencode(
        ext: *const c_char,
        inner: *const CMat,
        flag_ptr: *const ImageWriteMode,
        flag_size: usize,
    ) -> ImencodeResult;
}

// =============================================================================
//  Imgproc
// =============================================================================
/// ImreadModes. [See documentation](https://docs.opencv.org/trunk/d4/da8/group__imgcodecs.html#ga61d9b0126a3e57d9277ac48327799c80) for detauls
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ImageReadMode {
    /// If set, return the loaded image as is (with alpha channel, otherwise it
    /// gets cropped
    Unchanged = -1,
    /// If set, always convert image to the single channel grayscale image.
    Grayscale = 0,
    /// If set, always convert image to the 3 channel BGR color image.
    Color = 1,
    /// If set, return 16-bit/32-bit image when the input has the corresponding
    /// depth, otherwise convert it to 8-bit.
    AnyDepth = 2,
    /// If set, the image is read in any possible color format.
    AnyColor = 4,
    /// If set, use the gdal driver for loading the image.
    LoadGdal = 8,
    /// If set, always convert image to the single channel grayscale image and
    /// the image size reduced 1/2.
    ReducedGrayscale2 = 16,
    /// If set, always convert image to the 3 channel BGR color image and the
    /// image size reduced 1/2.
    ReducedColor2 = 17,
    /// If set, always convert image to the single channel grayscale image and
    /// the image size reduced 1/4.
    ReducedGrayscale4 = 32,
    /// If set, always convert image to the 3 channel BGR color image and the
    /// image size reduced 1/4.
    ReducedColor4 = 33,
    /// If set, always convert image to the single channel grayscale image and
    /// the image size reduced 1/8.
    ReducedGrayscale8 = 64,
    /// If set, always convert image to the 3 channel BGR color image and the
    /// image size reduced 1/8.
    ReducedColor8 = 65,
}

/// Imwrite flags. [See documentation](https://docs.opencv.org/trunk/d4/da8/group__imgcodecs.html#ga292d81be8d76901bff7988d18d2b42ac) for detauls
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ImageWriteMode {
    /// For JPEG, it can be a quality from 0 to 100 (the higher is the
    /// better). Default value is 95.
    JpegQuality = 1,
    /// Enable JPEG features, 0 or 1, default is False.
    JpegProgressive = 2,
    /// Enable JPEG features, 0 or 1, default is False.
    JpegOptimize = 3,
    /// JPEG restart interval, 0 - 65535, default is 0 - no restart.
    JpegRstInterval = 4,
    /// Separate luma quality level, 0 - 100, default is 0 - don't use.
    JpegLumaQuality = 5,
    /// Separate chroma quality level, 0 - 100, default is 0 - don't use.
    JpegChromaQuality = 6,
    /// For PNG, it can be the compression level from 0 to 9. A higher value
    /// means a smaller size and longer compression time. Default value is 3.
    /// Also strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT
    /// (Z_DEFAULT_STRATEGY).
    PngCompression = 16,
    /// One of cv::ImwritePNGFlags, default is IMWRITE_PNG_STRATEGY_DEFAULT.
    PngStrategy = 17,
    /// Binary level PNG, 0 or 1, default is 0.
    PngBilevel = 18,
    /// For PPM, PGM, or PBM, it can be a binary format flag, 0 or 1. Default
    /// value is 1.
    PxmBinary = 32,
    /// For WEBP, it can be a quality from 1 to 100 (the higher is the
    /// better). By default (without any parameter) and for quality above 100
    /// the lossless compression is used.
    WebpQuality = 64,
    /// For PAM, sets the TUPLETYPE field to the corresponding string value that
    /// is defined for the format
    PamTupletype = 128,
}

/// Imwrite PNG flag. [See documentation](https://docs.opencv.org/3.3.0/d4/da8/group__imgcodecs.html#gaa60044d347ffd187161b5ec9ea2ef2f9) for detauls
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ImageWritePngStrategy {
    /// Use this value for normal data.
    Default = 0,
    ///  Use this value for data produced by a filter (or predictor).Filtered
    ///  data consists mostly of small values with a somewhat random
    ///  distribution. In this case, the compression algorithm is tuned to
    ///  compress them better.
    Filtered = 1,
    /// Use this value to force Huffman encoding only (no string match).
    HuffmanOnly = 2,
    /// Use this value to limit match distances to one (run-length encoding).
    RLE = 3,
    /// Using this value prevents the use of dynamic Huffman codes, allowing for
    /// a simpler decoder for special applications.
    Fixed = 4,
}

#[repr(C)]
struct ImencodeResult {
    status: bool,
    buf: *mut u8,
    size: usize,
}

impl Mat {
    /// Decodes an image from `buf` according to the specified mode.
    pub fn image_decode(buf: &[u8], mode: ImageReadMode) -> Mat {
        let inner = unsafe { cv_imdecode(buf.as_ptr(), buf.len(), mode) };
        Self::from_raw(inner)
    }

    /// Encodes an image; the encoding scheme depends on the extension provided;
    /// additional write flags can be passed in using a vector. If successful,
    /// returns an owned vector of the encoded image.
    pub fn image_encode(&self, ext: &str, flags: Vec<ImageWriteMode>) -> Result<Vec<u8>, Error> {
        let ext = CString::new(ext)?;
        let r = unsafe { cv_imencode(ext.into_raw(), self.inner, flags.as_ptr(), flags.len()) };
        if r.status {
            unsafe { Ok(::std::slice::from_raw_parts(r.buf, r.size).to_vec()) }
        } else {
            Err(CvError::UnknownError("Unable to convert this image to bytes".into()).into())
        }
    }

    /// Creates a `Mat` from reading the image specified by the path.
    pub fn from_path<P: AsRef<Path>>(path: P, flags: ImageReadMode) -> Result<Mat, Error> {
        let path = path_to_cstring(path)?;
        let path = path.as_ptr();
        let result = unsafe { cv_imread(path, flags) };
        Ok(Mat::from_raw(result))
    }
}
