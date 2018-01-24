//! Image processing, see [OpenCV
//! imgproc](http://docs.opencv.org/3.1.0/d7/dbd/group__imgproc.html).

use super::*;
use super::core::*;
use std::os::raw::{c_double, c_float, c_int};
use num::ToPrimitive;
use std::borrow::Cow;

// =============================================================================
//  Imgproc
// =============================================================================
extern "C" {
    fn cv_line(
        cmat: *mut CMat,
        pt1: Point2i,
        pt2: Point2i,
        color: Scalar,
        thickness: c_int,
        linetype: c_int,
        shift: c_int,
    );

    fn cv_rectangle(cmat: *mut CMat, rect: Rect, color: Scalar, thickness: c_int, linetype: c_int);

    fn cv_ellipse(
        cmat: *mut CMat,
        center: Point2i,
        axes: Size2i,
        angle: c_double,
        start_angle: c_double,
        end_angle: c_double,
        color: Scalar,
        thickness: c_int,
        linetype: c_int,
        shift: c_int,
    );

    fn cv_cvt_color(cmat: *const CMat, output: *mut CMat, code: i32);
    fn cv_pyr_down(cmat: *const CMat, output: *mut CMat);
    fn cv_resize(from: *const CMat, to: *mut CMat, dsize: Size2i, fx: c_double, fy: c_double, interpolation: c_int);
    fn cv_calc_hist(
        cimages: *const CMat,
        nimages: i32,
        channels: *const c_int,
        cmask: *const CMat,
        chist: *mut CMat,
        dims: c_int,
        hist_size: *const c_int,
        ranges: *const *const c_float,
    );
    fn cv_calc_back_project(
        cimages: *const CMat,
        nimages: c_int,
        channels: *const c_int,
        chist: *const CMat,
        cback_project: *mut CMat,
        ranges: *const *const c_float,
    );

    fn cv_compare_hist(first_image: *const CMat, second_image: *const CMat, method: c_int) -> CResult<c_double>;
}

/// Possible methods for histogram comparision method
#[derive(Debug, PartialEq, Clone, Copy, ToPrimitive)]
pub enum HistogramComparisionMethod {
    /// HISTCMP_CORREL
    Corellation = 0,
    /// HISTCMP_CHISQR
    ChiSquare = 1,
    /// HISTCMP_INTERSECT
    Intersection = 2,
    /// HISTCMP_BHATTACHARYYA **and** HISTCMP_HELLINGER
    Bhattacharyya = 3,
    /// HISTCMP_CHISQR_ALT
    ChiSquareAlternative = 4,
    /// HISTCMP_KL_DIV
    KullbackLeiblerDivergence = 5
}

/// Color conversion code used in
/// [cvt_color](../struct.Mat.html#method.cvt_color).
#[allow(non_camel_case_types, missing_docs)]
#[derive(Debug, PartialEq, Clone, Copy)]
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

/// Interpolation algorithm
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InterpolationFlag {
    /// nearest neighbor interpolation
    InterNearst = 0,

    /// bilinear interpolation
    InterLinear = 1,

    /// bicubic interpolation
    InterCubic = 2,

    /// resampling using pixel area relation. It may be a preferred method for
    /// image decimation, as it gives moire'-free results. But when the image is
    /// zoomed, it is similar to the INTER_NEAREST method.
    InterArea = 3,

    /// Lanczos interpolation over 8x8 neighborhood
    InterLanczos4 = 4,

    /// mask for interpolation codes
    InterMax = 7,

    /// flag, fills all of the destination image pixels. If some of them
    /// correspond to outliers in the source image, they are set to zero
    WarpFillOutliers = 8,

    /// flag, inverse transformation
    WarpInverseMap = 16,
}

impl Mat {
    /// Draws a simple line.
    pub fn line(&self, pt1: Point2i, pt2: Point2i) {
        let color = Scalar::new(255, 255, 0, 255);
        self.line_custom(pt1, pt2, color, 1, LineTypes::Line8, 0);
    }

    /// Draws a line with custom color, thickness and linetype.
    pub fn line_custom(
        &self,
        pt1: Point2i,
        pt2: Point2i,
        color: Scalar,
        thickness: i32,
        linetype: LineTypes,
        shift: i32,
    ) {
        unsafe {
            cv_line(
                self.inner,
                pt1,
                pt2,
                color,
                thickness,
                linetype as i32,
                shift,
            );
        }
    }

    /// Draws a simple, thick, or filled up-right rectangle.
    pub fn rectangle(&self, rect: Rect) {
        self.rectangle_custom(rect, Scalar::new(255, 255, 0, 255), 1, LineTypes::Line8);
    }

    /// Draws a rectangle with custom color, thickness and linetype.
    pub fn rectangle_custom(&self, rect: Rect, color: Scalar, thickness: i32, linetype: LineTypes) {
        unsafe { cv_rectangle(self.inner, rect, color, thickness, linetype as i32) }
    }

    /// Draw a simple, thick, or filled up-right rectangle.
    pub fn rectangle2f(&self, rect: Rect2f) {
        let abs_rect = rect.normalize_to_mat(self);
        self.rectangle(abs_rect);
    }

    /// Draws a simple, thick ellipse
    pub fn ellipse(&self, center: Point2i, axes: Size2i, angle: f64, start_angle: f64, end_angle: f64) {
        self.ellipse_custom(
            center,
            axes,
            angle,
            start_angle,
            end_angle,
            Scalar::new(255, 255, 0, 255),
            1,
            LineTypes::Line8,
            0,
        )
    }

    /// Draws a custom ellipse
    pub fn ellipse_custom(
        &self,
        center: Point2i,
        axes: Size2i,
        angle: f64,
        start_angle: f64,
        end_angle: f64,
        color: Scalar,
        thickness: i32,
        linetype: LineTypes,
        shift: i32,
    ) {
        unsafe {
            cv_ellipse(
                self.inner,
                center,
                axes,
                angle,
                start_angle,
                end_angle,
                color,
                thickness,
                linetype as i32,
                shift,
            )
        }
    }

    /// Convert an image from one color space to another.
    pub fn cvt_color(&self, code: ColorConversionCodes) -> Mat {
        let m = CMat::new();
        unsafe { cv_cvt_color(self.inner, m, code as i32) }
        Mat::from_raw(m)
    }

    /// Blurs an image and downsamples it. This function performs the
    /// downsampling step of the Gaussian pyramid construction.
    pub fn pyr_down(&self) -> Mat {
        let m = CMat::new();
        unsafe { cv_pyr_down(self.inner, m) }
        Mat::from_raw(m)
    }

    /// Resizes an image.
    ///
    /// The function resize resizes the image down to or up to the specified
    /// size.
    pub fn resize_to(&self, dsize: Size2i, interpolation: InterpolationFlag) -> Mat {
        let m = CMat::new();
        unsafe { cv_resize(self.inner, m, dsize, 0.0, 0.0, interpolation as c_int) }
        Mat::from_raw(m)
    }

    /// Resizes an image.
    ///
    /// The function resize resizes the image down to or up to the specified
    /// size.
    pub fn resize_by(&self, fx: f64, fy: f64, interpolation: InterpolationFlag) -> Mat {
        let m = CMat::new();
        unsafe {
            cv_resize(
                self.inner,
                m,
                Size2i::default(),
                fx,
                fy,
                interpolation as c_int,
            )
        }
        Mat::from_raw(m)
    }

    /// Calculate a histogram of an image.
    pub fn calc_hist(
        &self,
        channels: *const c_int,
        mask: Mat,
        dims: c_int,
        hist_size: *const c_int,
        ranges: *const *const f32,
    ) -> Mat {
        let m = CMat::new();
        unsafe {
            cv_calc_hist(
                self.inner,
                1,
                channels,
                mask.inner,
                m,
                dims,
                hist_size,
                ranges,
            );
        }
        Mat::from_raw(m)
    }

    /// Calculate the back projection of a histogram. The function calculates
    /// the back project of the histogram.
    pub fn calc_back_project(&self, channels: *const i32, hist: &Mat, ranges: *const *const f32) -> Mat {
        let m = CMat::new();
        unsafe {
            cv_calc_back_project(self.inner, 1, channels, (*hist).inner, m, ranges);
        }
        Mat::from_raw(m)
    }
}

impl Cv {
    /// Compares two histograms.
    /// The function compare two histograms using the specified method.
    /// The function returns d(first_image, second_image).
    /// While the function works well with 1-, 2-, 3-dimensional dense histograms, it may not be
    /// suitable for high-dimensional sparse histograms.
    /// In such histograms, because of aliasing and sampling problems,
    /// the coordinates of non-zero histogram bins can slightly shift.
    /// To compare such histograms or more general sparse configurations of weighted points,
    /// consider using the cv::EMD function.
    pub fn compare_hist(first_image: &Mat, second_image: &Mat, method: HistogramComparisionMethod) -> Result<f64, Cow<'static, str>> {
        let method: c_int = method.to_i64().unwrap() as i32;
        let result = unsafe {
            cv_compare_hist(first_image.inner, second_image.inner, method)
        };
        if result.error.is_null() {
            Ok(result.value)
        }
        else {
            unsafe {
                let c_str = std::ffi::CStr::from_ptr(result.error);
                let err = match c_str.to_str() {
                    Ok(message) => message.into(),
                    _ => "Unknown error".into()
                };
                Err(err)
            }
        }
    }
}