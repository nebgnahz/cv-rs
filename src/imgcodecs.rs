//! Image file reading and writing, see [OpenCV
//! imgcodecs](http://docs.opencv.org/3.1.0/d4/da8/group__imgcodecs.html).

use core::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::path::Path;
use failure::Error;

// =============================================================================
//  Imgproc
// =============================================================================
/// ImreadModes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImreadModes {
    /// If set, return the loaded image as is (with alpha channel, otherwise it
    /// gets cropped
    ImreadUnchanged = -1,
    /// If set, always convert image to the single channel grayscale image.
    ImreadGrayscale = 0,
    /// If set, always convert image to the 3 channel BGR color image.
    ImreadColor = 1,
    /// If set, return 16-bit/32-bit image when the input has the corresponding
    /// depth, otherwise convert it to 8-bit.
    ImreadAnydepth = 2,
    /// If set, the image is read in any possible color format.
    ImreadAnycolor = 4,
    /// If set, use the gdal driver for loading the image.
    ImreadLoadGdal = 8,
    /// If set, always convert image to the single channel grayscale image and
    /// the image size reduced 1/2.
    ImreadReducedGrayscale2 = 16,
    /// If set, always convert image to the 3 channel BGR color image and the
    /// image size reduced 1/2.
    ImreadReducedColor2 = 17,
    /// If set, always convert image to the single channel grayscale image and
    /// the image size reduced 1/4.
    ImreadReducedGrayscale4 = 32,
    /// If set, always convert image to the 3 channel BGR color image and the
    /// image size reduced 1/4.
    ImreadReducedColor4 = 33,
    /// If set, always convert image to the single channel grayscale image and
    /// the image size reduced 1/8.
    ImreadReducedGrayscale8 = 64,
    /// If set, always convert image to the 3 channel BGR color image and the
    /// image size reduced 1/8.
    ImreadReducedColor8 = 65,
}

/// Imwrite flags
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImwriteFlags {
    /// For JPEG, it can be a quality from 0 to 100 (the higher is the
    /// better). Default value is 95.
    ImwriteJpegQuality = 1,
    /// Enable JPEG features, 0 or 1, default is False.
    ImwriteJpegProgressive = 2,
    /// Enable JPEG features, 0 or 1, default is False.
    ImwriteJpegOptimize = 3,
    /// JPEG restart interval, 0 - 65535, default is 0 - no restart.
    ImwriteJpegRstInterval = 4,
    /// Separate luma quality level, 0 - 100, default is 0 - don't use.
    ImwriteJpegLumaQuality = 5,
    /// Separate chroma quality level, 0 - 100, default is 0 - don't use.
    ImwriteJpegChromaQuality = 6,
    /// For PNG, it can be the compression level from 0 to 9. A higher value
    /// means a smaller size and longer compression time. Default value is 3.
    /// Also strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT
    /// (Z_DEFAULT_STRATEGY).
    ImwritePngCompression = 16,
    /// One of cv::ImwritePNGFlags, default is IMWRITE_PNG_STRATEGY_DEFAULT.
    ImwritePngStrategy = 17,
    /// Binary level PNG, 0 or 1, default is 0.
    ImwritePngBilevel = 18,
    /// For PPM, PGM, or PBM, it can be a binary format flag, 0 or 1. Default
    /// value is 1.
    ImwritePxmBinary = 32,
    /// For WEBP, it can be a quality from 1 to 100 (the higher is the
    /// better). By default (without any parameter) and for quality above 100
    /// the lossless compression is used.
    ImwriteWebpQuality = 64,
    /// For PAM, sets the TUPLETYPE field to the corresponding string value that
    /// is defined for the format
    ImwritePamTupletype = 128,
}

/// Imwrite PNG flag
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImwritePngFlags {
    /// Use this value for normal data.
    ImwritePngStrategyDefault = 0,
    ///  Use this value for data produced by a filter (or predictor).Filtered
    ///  data consists mostly of small values with a somewhat random
    ///  distribution. In this case, the compression algorithm is tuned to
    ///  compress them better.
    ImwritePngStrategyFiltered = 1,
    /// Use this value to force Huffman encoding only (no string match).
    ImwritePngStrategyHuffmanOnly = 2,
    /// Use this value to limit match distances to one (run-length encoding).
    ImwritePngStrategyRle = 3,
    /// Using this value prevents the use of dynamic Huffman codes, allowing for
    /// a simpler decoder for special applications.
    ImwritePngStrategyFixed = 4,
}

extern "C" {
    fn cv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn cv_imdecode(buf: *const u8, l: usize, m: c_int) -> *mut CMat;
    fn cv_imencode(ext: *const c_char, inner: *const CMat, flag_ptr: *const c_int, flag_size: usize) -> ImencodeResult;

}

#[repr(C)]
struct ImencodeResult {
    status: bool,
    buf: *mut u8,
    size: usize,
}

impl Mat {
    /// Creates a `Mat` from reading the image specified by the path.
    pub fn from_path<P: AsRef<Path>>(path: P, flags: ImreadModes) -> Result<Mat, Error> {
        let path = path_to_cstring(path)?;
        let path = path.as_ptr();
        let result = unsafe { cv_imread(path, flags as c_int) };
        Ok(Mat::from_raw(result))
    }

    /// Decodes an image from `buf` according to the specified mode.
    pub fn imdecode(buf: &[u8], mode: ImreadModes) -> Mat {
        let inner = unsafe { cv_imdecode(buf.as_ptr(), buf.len(), mode as c_int) };
        Mat::from_raw(inner)
    }

    /// Encodes an image; the encoding scheme depends on the extension provided;
    /// additional write flags can be passed in using a vector. If successful,
    /// returns an owned vector of the encoded image.
    pub fn imencode(&self, ext: &str, f: Vec<ImwriteFlags>) -> Option<Vec<u8>> {
        let ext = CString::new(ext).expect("invalid extension string");
        let flags = f.into_iter().map(|f| f as c_int).collect::<Vec<_>>();
        let r = unsafe { cv_imencode(ext.into_raw(), self.inner, flags.as_ptr(), flags.len()) };
        if r.status {
            unsafe { Some(::std::slice::from_raw_parts(r.buf, r.size).to_vec()) }
        } else {
            None
        }
    }
}
