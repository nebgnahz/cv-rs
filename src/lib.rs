//! A library that integrates computer vision algorithms, mostly OpenCV.
//!
//! This is a work-in-progress port of OpenCV 3.1.0. Modules and functions are
//! implemented as needed. There is another library
//! [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
//! bindings using a Python script.
extern crate libc;
use libc::{c_double, c_int};
use std::ffi::CString;
use std::os::raw::c_char;

mod highgui;
pub use highgui::highgui_named_window;
pub use highgui::highgui_destroy_window;
pub use highgui::highgui_set_mouse_callback;
pub use highgui::MouseCallback;
pub use highgui::MouseCallbackData;
pub use highgui::MouseEventTypes;
pub use highgui::WindowFlags;

mod core;
pub use core::Scalar;
pub use core::Point2i;
pub use core::Point2f;
pub use core::Size2i;
pub use core::Size2f;
pub use core::Rect;
use core::CVecOfRect;
pub use core::VecOfRect;
use core::CMat;
use core::opencv_mat_new;
pub use core::Mat;

pub use core::FlipCode;

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
        unsafe { opencv_in_range(self.c_mat, lowerb, upperb, m) }
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
            opencv_mix_channels(self.c_mat,
                                nsrcs,
                                m.c_mat,
                                ndsts,
                                from_to,
                                npairs);
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormTypes) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_normalize(self.c_mat, m, alpha, beta, t as i32) }
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

enum CCascadeClassifier {}

/// Cascade classifier class for object detection.
pub struct CascadeClassifier {
    c_cascade_classifier: *mut CCascadeClassifier,
}

extern "C" {
    fn opencv_cascade_classifier_new() -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_load(cc: *mut CCascadeClassifier,
                                      p: *const c_char)
                                      -> bool;
    fn opencv_cascade_classifier_from_path(p: *const c_char)
                                           -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_drop(p: *mut CCascadeClassifier);
    fn opencv_cascade_classifier_detect(cc: *mut CCascadeClassifier,
                                        cmat: *mut CMat,
                                        vec_of_rect: *mut CVecOfRect,
                                        scale_factor: c_double,
                                        min_neighbors: c_int,
                                        flags: c_int,
                                        min_size: Size2i,
                                        max_size: Size2i);
}

impl CascadeClassifier {
    /// Create a cascade classifier, uninitialized. Before use, call load.
    pub fn new() -> CascadeClassifier {
        let cascade = unsafe { opencv_cascade_classifier_new() };
        CascadeClassifier { c_cascade_classifier: cascade }
    }

    pub fn load(&self, path: &str) -> bool {
        let s = CString::new(path).unwrap();
        unsafe {
            opencv_cascade_classifier_load(self.c_cascade_classifier,
                                           (&s).as_ptr())
        }
    }

    pub fn from_path(path: &str) -> Self {
        let s = CString::new(path).unwrap();
        let cascade =
            unsafe { opencv_cascade_classifier_from_path((&s).as_ptr()) };
        CascadeClassifier { c_cascade_classifier: cascade }
    }

    pub fn detect(&self, mat: &Mat, result: &mut VecOfRect) {
        unsafe {
            opencv_cascade_classifier_detect(self.c_cascade_classifier,
                                             mat.get_cmat(),
                                             result.get_mut_c_vec_of_rec(),
                                             1.1,
                                             3,
                                             0,
                                             Size2i::default(),
                                             Size2i::default());
        }

        result.populate_rects();
    }

    pub fn detect_with_params(&self,
                              mat: &Mat,
                              result: &mut VecOfRect,
                              scale_factor: f64,
                              min_neighbors: i32,
                              min_size: Size2i,
                              max_size: Size2i) {
        unsafe {
            opencv_cascade_classifier_detect(self.c_cascade_classifier,
                                             mat.get_cmat(),
                                             result.get_mut_c_vec_of_rec(),
                                             scale_factor,
                                             min_neighbors,
                                             0,
                                             min_size,
                                             max_size);
        }

        result.populate_rects();
    }
}

impl Drop for CascadeClassifier {
    fn drop(&mut self) {
        unsafe {
            opencv_cascade_classifier_drop(self.c_cascade_classifier);
        }
    }
}

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
        unsafe { opencv_camshift(self.c_mat, wndw, criteria.c_criteria) }
    }
}
