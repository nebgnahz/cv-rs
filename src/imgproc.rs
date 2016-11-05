extern crate libc;

use libc::{c_float, c_int};
use super::core::*;

// =============================================================================
//  Imgproc
// =============================================================================
extern "C" {
    fn opencv_rectangle(cmat: *mut CMat, rect: Rect);
    fn opencv_cvt_color(cmat: *const CMat, output: *mut CMat, code: i32);
    fn opencv_pyr_down(cmat: *const CMat, output: *mut CMat);
    fn opencv_calc_hist(cimages: *const CMat,
                        nimages: i32,
                        channels: *const c_int,
                        cmask: *const CMat,
                        chist: *mut CMat,
                        dims: c_int,
                        hist_size: *const c_int,
                        ranges: *const *const c_float);
    fn opencv_calc_back_project(cimages: *const CMat,
                                nimages: c_int,
                                channels: *const c_int,
                                chist: *const CMat,
                                cback_project: *mut CMat,
                                ranges: *const *const c_float);
}

/// Color conversion code used in [cvt_color](struct.Mat.html#method.cvt_color).
#[allow(non_camel_case_types)]
pub enum ColorConversionCodes {
    BGR2BGRA = 0,
    BGRA2BGR = 1,
    BGR2RGBA = 2,
    RGBA2BGR = 3,
    BGR2RGB = 4,
    BGRA2RGBA = 5,
    BGR2GRAY = 6,
    RGB2GRAY = 7,
    GRAY2BGR = 8,
    GRAY2BGRA = 9,
    BGRA2GRAY = 10,
    RGBA2GRAY = 11,
    BGR2BGR565 = 12,
    RGB2BGR565 = 13,
    BGR5652BGR = 14,
    BGR5652RGB = 15,
    BGRA2BGR565 = 16,
    RGBA2BGR565 = 17,
    BGR5652BGRA = 18,
    BGR5652RGBA = 19,
    GRAY2BGR565 = 20,
    BGR5652GRAY = 21,
    BGR2BGR555 = 22,
    RGB2BGR555 = 23,
    BGR5552BGR = 24,
    BGR5552RGB = 25,
    BGRA2BGR555 = 26,
    RGBA2BGR555 = 27,
    BGR5552BGRA = 28,
    BGR5552RGBA = 29,
    GRAY2BGR555 = 30,
    BGR5552GRAY = 31,
    BGR2XYZ = 32,
    RGB2XYZ = 33,
    XYZ2BGR = 34,
    XYZ2RGB = 35,
    BGR2YCrCb = 36,
    RGB2YCrCb = 37,
    YCrCb2BGR = 38,
    YCrCb2RGB = 39,
    BGR2HSV = 40,
    RGB2HSV = 41,
    BGR2Lab = 44,
    RGB2Lab = 45,
    BGR2Luv = 50,
    RGB2Luv = 51,
    BGR2HLS = 52,
    RGB2HLS = 53,
    HSV2BGR = 54,
    HSV2RGB = 55,
    Lab2BGR = 56,
    Lab2RGB = 57,
    Luv2BGR = 58,
    Luv2RGB = 59,
    HLS2BGR = 60,
    HLS2RGB = 61,
    BGR2HSV_FULL = 66,
    RGB2HSV_FULL = 67,
    BGR2HLS_FULL = 68,
    RGB2HLS_FULL = 69,
    HSV2BGR_FULL = 70,
    HSV2RGB_FULL = 71,
    HLS2BGR_FULL = 72,
    HLS2RGB_FULL = 73,
    LBGR2Lab = 74,
    LRGB2Lab = 75,
    LBGR2Luv = 76,
    LRGB2Luv = 77,
    Lab2LBGR = 78,
    Lab2LRGB = 79,
    Luv2LBGR = 80,
    Luv2LRGB = 81,
    BGR2YUV = 82,
    RGB2YUV = 83,
    YUV2BGR = 84,
    YUV2RGB = 85,
    YUV2RGB_NV12 = 90,
    YUV2BGR_NV12 = 91,
    YUV2RGB_NV21 = 92,
    YUV2BGR_NV21 = 93,
    YUV2RGBA_NV12 = 94,
    YUV2BGRA_NV12 = 95,
    YUV2RGBA_NV21 = 96,
    YUV2BGRA_NV21 = 97,
    YUV2RGB_YV12 = 98,
    YUV2BGR_YV12 = 99,
    YUV2RGB_IYUV = 100,
    YUV2BGR_IYUV = 101,
    YUV2RGBA_YV12 = 102,
    YUV2BGRA_YV12 = 103,
    YUV2RGBA_IYUV = 104,
    YUV2BGRA_IYUV = 105,
    YUV2GRAY_420 = 106,
    YUV2RGB_UYVY = 107,
    YUV2BGR_UYVY = 108,
    YUV2RGBA_UYVY = 111,
    YUV2BGRA_UYVY = 112,
    YUV2RGB_YUY2 = 115,
    YUV2BGR_YUY2 = 116,
    YUV2RGB_YVYU = 117,
    YUV2BGR_YVYU = 118,
    YUV2RGBA_YUY2 = 119,
    YUV2BGRA_YUY2 = 120,
    YUV2RGBA_YVYU = 121,
    YUV2BGRA_YVYU = 122,
    YUV2GRAY_UYVY = 123,
    YUV2GRAY_YUY2 = 124,
    RGBA2mRGBA = 125,
    mRGBA2RGBA = 126,
    RGB2YUV_I420 = 127,
    BGR2YUV_I420 = 128,
    RGBA2YUV_I420 = 129,
    BGRA2YUV_I420 = 130,
    RGB2YUV_YV12 = 131,
    BGR2YUV_YV12 = 132,
    RGBA2YUV_YV12 = 133,
    BGRA2YUV_YV12 = 134,
    BayerBG2BGR = 46,
    BayerGB2BGR = 47,
    BayerRG2BGR = 48,
    BayerGR2BGR = 49,
    BayerBG2GRAY = 86,
    BayerGB2GRAY = 87,
    BayerRG2GRAY = 88,
    BayerGR2GRAY = 89,
    BayerBG2BGR_VNG = 62,
    BayerGB2BGR_VNG = 63,
    BayerRG2BGR_VNG = 64,
    BayerGR2BGR_VNG = 65,
    BayerBG2BGR_EA = 135,
    BayerGB2BGR_EA = 136,
    BayerRG2BGR_EA = 137,
    BayerGR2BGR_EA = 138,
    COLORCVT_MAX = 139,
}

impl Mat {
    /// Draw a simple, thick, or filled up-right rectangle.
    pub fn rectangle(&self, rect: Rect) {
        unsafe {
            opencv_rectangle(self.inner, rect);
        }
    }

    /// Convert an image from one color space to another.
    pub fn cvt_color(&self, code: ColorConversionCodes) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_cvt_color(self.inner, m, code as i32) }
        Mat::new_with_cmat(m)
    }

    /// Convert an image from one color space to another.
    pub fn pyr_down(&self) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_pyr_down(self.inner, m) }
        Mat::new_with_cmat(m)
    }

    /// Calculate a histogram of an image.
    pub fn calc_hist(&self,
                     channels: *const c_int,
                     mask: Mat,
                     dims: c_int,
                     hist_size: *const c_int,
                     ranges: *const *const f32)
                     -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe {
            opencv_calc_hist(self.inner,
                             1,
                             channels,
                             mask.inner,
                             m,
                             dims,
                             hist_size,
                             ranges);
        }
        Mat::new_with_cmat(m)
    }

    /// Calculate the back projection of a histogram. The function calculates
    /// the back project of the histogram.
    pub fn calc_back_project(&self, channels: *const i32, hist: &Mat, ranges: *const *const f32) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe {
            opencv_calc_back_project(self.inner, 1, channels, (*hist).inner, m, ranges);
        }
        Mat::new_with_cmat(m)
    }
}
