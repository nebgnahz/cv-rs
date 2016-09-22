use super::core::{CMat, Mat};
use super::libc::{c_int, size_t, uint8_t};

// =============================================================================
//  Imgproc
// =============================================================================
/// ImreadModes
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
    /// means a smaller size and longer compression time. Default value is
    /// 3. Also strategy is changed to IMWRITE_PNG_STRATEGY_DEFAULT
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
    fn opencv_imdecode(buf: *const uint8_t, l: size_t, m: c_int) -> *mut CMat;
    fn opencv_imencode(ext: *const uint8_t,
                       c_mat: *const CMat,
                       flag_ptr: *const c_int,
                       flag_size: size_t)
                       -> ImencodeResult;

}

#[repr(C)]
struct ImencodeResult {
    status: bool,
    buf: *mut u8,
    size: usize,
}

impl Mat {
    pub fn imdecode(buf: &[u8], mode: ImreadModes) -> Mat {
        let c_mat =
            unsafe { opencv_imdecode(buf.as_ptr(), buf.len(), mode as i32) };
        Mat::new_with_cmat(c_mat)
    }

    pub fn imencode(&self, ext: &str, f: Vec<ImwriteFlags>) -> Option<Vec<u8>> {
        let flags = f.into_iter().map(|f| f as i32).collect::<Vec<_>>();
        let r = unsafe {
            opencv_imencode(ext.as_ptr(),
                            self.c_mat,
                            flags.as_ptr(),
                            flags.len())
        };
        if r.status {
            unsafe {
                Some(::std::slice::from_raw_parts(r.buf, r.size).to_vec())
            }
        } else {
            None
        }
    }
}
