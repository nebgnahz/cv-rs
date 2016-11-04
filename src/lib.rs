//! This library primarily provides a binding and API for OpenCV 3.1.0.
//!
//! This is a work-in-progress and modules/functions are implemented as
//! needed. Attempts to use
//! [rust-bindgen](https://github.com/servo/rust-bindgen) or
//! [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust) haven't been very
//! successful (I probably haven't tried hard enough). There is another port
//! [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
//! bindings using a Python script.
extern crate libc;
use libc::{c_double, c_int};

mod highgui;
pub use highgui::MouseCallback;
pub use highgui::MouseCallbackData;
pub use highgui::MouseEventTypes;
pub use highgui::WindowFlags;
pub use highgui::highgui_destroy_window;
pub use highgui::highgui_named_window;
pub use highgui::highgui_set_mouse_callback;

mod core;
use core::CMat;

pub use core::FlipCode;
pub use core::Mat;
pub use core::Point2f;
pub use core::Point2i;
pub use core::Rect;
pub use core::Scalar;
pub use core::Size2f;
pub use core::Size2i;
use core::opencv_mat_new;

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
        let angle = self.angle * std::f32::consts::PI / 180.0;

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

        let width =
            pt.iter().map(|p| p.x).fold(0. / 0., f32::max).ceil() as i32 - x +
            1;
        let height =
            pt.iter().map(|p| p.y).fold(0. / 0., f32::max).ceil() as i32 - y +
            1;
        Rect::new(x, y, width, height)
    }
}

// =============================================================================
//  core array
// =============================================================================
extern "C" {
    fn opencv_in_range(cmat: *const CMat,
                       lowerb: Scalar,
                       upperb: Scalar,
                       dst: *mut CMat);
    fn opencv_mix_channels(cmat: *const CMat,
                           nsrcs: isize,
                           dst: *mut CMat,
                           ndsts: isize,
                           from_to: *const i32,
                           npairs: isize);
    fn opencv_normalize(csrc: *const CMat,
                        cdst: *mut CMat,
                        alpha: c_double,
                        beta: c_double,
                        norm_type: c_int);
}

pub enum NormTypes {
    NormInf = 1,
    NormL1 = 2,
    NormL2 = 4,
    NormL2Sqr = 5,
    NormHamming = 6,
    NormHamming2 = 7,
    NormRelative = 8,
    NormMinMax = 32,
}

impl Mat {
    /// Check if Mat elements lie between the elements of two other arrays
    /// (lowerb and upperb). The output Mat has the same size as `self` and
    /// CV_8U type.
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_in_range(self.inner, lowerb, upperb, m) }
        Mat::new_with_cmat(m)
    }

    /// Copy specified channels from `self` to the specified channels of output
    /// `Mat`.
    // TODO(benzh) Avoid using raw pointers but rather take a vec for `from_to`?
    pub fn mix_channels(&self,
                        nsrcs: isize,
                        ndsts: isize,
                        from_to: *const i32,
                        npairs: isize)
                        -> Mat {
        let m = Mat::with_size(self.rows, self.cols, self.depth);
        unsafe {
            opencv_mix_channels(self.inner,
                                nsrcs,
                                m.inner,
                                ndsts,
                                from_to,
                                npairs);
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormTypes) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_normalize(self.inner, m, alpha, beta, t as i32) }
        Mat::new_with_cmat(m)
    }
}

mod imgcodecs;
pub use imgcodecs::{ImreadModes, ImwriteFlags, ImwritePngFlags};

mod imgproc;
pub use imgproc::ColorConversionCodes;

// =============================================================================
//   VideoCapture
// =============================================================================
enum CVideoCapture {}
pub struct VideoCapture {
    c_videocapture: *mut CVideoCapture,
}

extern "C" {
    fn opencv_videocapture_new(index: c_int) -> *mut CVideoCapture;
    fn opencv_videocapture_is_opened(ccap: *const CVideoCapture) -> bool;
    fn opencv_videocapture_read(v: *mut CVideoCapture, m: *mut CMat) -> bool;
    fn opencv_videocapture_drop(ccap: *mut CVideoCapture);
}

impl VideoCapture {
    pub fn new(index: i32) -> Self {
        let cap = unsafe { opencv_videocapture_new(index) };
        VideoCapture { c_videocapture: cap }
    }

    pub fn is_open(&self) -> bool {
        unsafe { opencv_videocapture_is_opened(self.c_videocapture) }
    }

    pub fn read(&self, mat: &Mat) -> bool {
        unsafe { opencv_videocapture_read(self.c_videocapture, mat.get_cmat()) }
    }
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        unsafe {
            opencv_videocapture_drop(self.c_videocapture);
        }
    }
}

pub mod objdetect;

// =============================================================================
//   VideoTrack
// =============================================================================
enum CTermCriteria {}
pub enum TermType {
    Count = 1,
    EPS = 2,
}

extern "C" {
    fn opencv_term_criteria_new(t: i32,
                                count: i32,
                                epsilon: f64)
                                -> *mut CTermCriteria;
    fn opencv_term_criteria_drop(criteria: *mut CTermCriteria);
    fn opencv_camshift(image: *mut CMat,
                       w: Rect,
                       c_criteria: *const CTermCriteria)
                       -> RotatedRect;
}

pub struct TermCriteria {
    c_criteria: *mut CTermCriteria,
}

impl TermCriteria {
    pub fn new(t: TermType, max_count: i32, epsilon: f64) -> Self {
        let c_criteria =
            unsafe { opencv_term_criteria_new(t as i32, max_count, epsilon) };
        TermCriteria { c_criteria: c_criteria }
    }
}

impl Drop for TermCriteria {
    fn drop(&mut self) {
        unsafe {
            opencv_term_criteria_drop(self.c_criteria);
        }
    }
}

impl Mat {
    pub fn camshift(&self, wndw: Rect, criteria: &TermCriteria) -> RotatedRect {
        unsafe { opencv_camshift(self.inner, wndw, criteria.c_criteria) }
    }
}
