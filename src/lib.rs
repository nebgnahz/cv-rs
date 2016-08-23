//! A Rust wrapper for OpenCV.

extern crate libc;
use libc::{c_double, c_float, c_int, c_void};
use std::ffi::CString;
use std::os::raw::c_char;

pub type CVoid = c_void;

type CMat = c_void;
pub struct Mat {
    c_mat: *mut CMat,
    cols: i32,
    rows: i32,
    depth: i32,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Scalar {
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
}

impl Scalar {
    pub fn new(v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
        Scalar {
            v0: v0,
            v1: v1,
            v2: v2,
            v3: v3,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2i {
    x: i32,
    y: i32,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2f {
    x: f32,
    y: f32,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2f {
    width: f32,
    height: f32,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct RotatedRect {
    center: Point2f,
    size: Size2f,
    angle: f32,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
struct CVecOfRect {
    array: *mut Rect,
    size: usize,
}

pub struct VecOfRect {
    rects: Vec<Rect>,
    c_vec_of_rect: CVecOfRect,
}

impl VecOfRect {
    pub fn draw_on_mat(&self, mat: &mut Mat) {
        self.rects.iter().map(|&r| mat.rectangle(r)).count();
    }

    fn get_mut_c_vec_of_rec(&mut self) -> &mut CVecOfRect {
        &mut self.c_vec_of_rect
    }

    pub fn populate_rects(&mut self) {
        for i in 0..self.c_vec_of_rect.size {
            let rect =
                unsafe { *(self.c_vec_of_rect.array.offset(i as isize)) };
            self.rects.push(rect);
        }
    }
}

impl Default for VecOfRect {
    fn default() -> Self {
        VecOfRect {
            c_vec_of_rect: CVecOfRect {
                array: std::ptr::null_mut::<Rect>(),
                size: 0,
            },
            rects: Vec::new(),
        }
    }
}

impl std::fmt::Debug for VecOfRect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " {:?} ", self.rects)
    }
}

impl Drop for VecOfRect {
    fn drop(&mut self) {
        extern "C" {
            fn opencv_vec_of_rect_drop(_: *mut CVecOfRect);
        }
        unsafe {
            opencv_vec_of_rect_drop(&mut self.c_vec_of_rect);
        }
    }
}

extern "C" {
    fn opencv_mat_new() -> *mut CMat;
    fn opencv_mat_new_with_size(rows: i32, cols: i32, t: i32) -> *mut CMat;
    fn opencv_mat_is_valid(mat: *mut CMat) -> bool;
    fn opencv_mat_rows(cmat: *const CMat) -> i32;
    fn opencv_mat_cols(cmat: *const CMat) -> i32;
    fn opencv_mat_depth(cmat: *const CMat) -> i32;
    fn opencv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn opencv_mat_roi(cmat: *const CMat, rect: Rect) -> *mut CMat;
    fn opencv_mat_logic_and(cimage: *mut CMat, cmask: *const CMat);
    fn opencv_mat_drop(mat: *mut CMat);
}

impl Mat {
    fn new_with_cmat(cmat: *mut CMat) -> Self {
        Mat {
            c_mat: cmat,
            rows: unsafe { opencv_mat_rows(cmat) },
            cols: unsafe { opencv_mat_cols(cmat) },
            depth: unsafe { opencv_mat_depth(cmat) },
        }
    }

    pub fn new() -> Self {
        let m = unsafe { opencv_mat_new() };
        Mat::new_with_cmat(m)
    }

    pub fn with_size(rows: i32, cols: i32, t: i32) -> Self {
        let m = unsafe { opencv_mat_new_with_size(rows, cols, t) };
        Mat::new_with_cmat(m)
    }

    pub fn from_path(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { opencv_imread((&s).as_ptr(), flags) };
        Mat::new_with_cmat(m)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { opencv_mat_is_valid(self.c_mat) }
    }

    pub fn roi(&self, rect: Rect) -> Mat {
        let cmat = unsafe { opencv_mat_roi(self.c_mat, rect) };
        Mat::new_with_cmat(cmat)
    }

    // TODO(benzh): Find the right reference in OpenCV for this one. Provide a
    // shortcut for `image &= mask`
    pub fn logic_and(&mut self, mask: Mat) {
        unsafe {
            opencv_mat_logic_and(self.c_mat, mask.get_cmat());
        }
    }

    pub fn show(&self, name: &str, delay: i32) {
        let s = CString::new(name).unwrap();
        unsafe {
            opencv_imshow((&s).as_ptr(), self.c_mat);
            opencv_wait_key(delay);
        }
    }

    fn get_cmat(&self) -> *mut CMat {
        self.c_mat
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            opencv_mat_drop(self.c_mat);
        }
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
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_in_range(self.c_mat, lowerb, upperb, m) }
        Mat::new_with_cmat(m)
    }

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

    pub fn normalize(&self, alpha: f64, beta: f64, t: NormTypes) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_normalize(self.c_mat, m, alpha, beta, t as i32) }
        Mat::new_with_cmat(m)
    }
}

// =============================================================================
//  Imgproc
// =============================================================================
extern "C" {
    fn opencv_rectangle(cmat: *mut CMat, rect: Rect);
    fn opencv_cvt_color(cmat: *const CMat, output: *mut CMat, code: i32);
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

pub enum ColorConversionCodes {
    BGR2BGRA = 0,
    BGR2HSV = 40,
}

impl Mat {
    pub fn rectangle(&self, rect: Rect) {
        unsafe {
            opencv_rectangle(self.c_mat, rect);
        }
    }

    pub fn cvt_color(&self, code: ColorConversionCodes) -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe { opencv_cvt_color(self.c_mat, m, code as i32) }
        Mat::new_with_cmat(m)
    }

    pub fn calc_hist(&self,
                     channels: *const c_int,
                     mask: Mat,
                     dims: c_int,
                     hist_size: *const c_int,
                     ranges: *const *const f32)
                     -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe {
            opencv_calc_hist(self.c_mat,
                             1,
                             channels,
                             mask.c_mat,
                             m,
                             dims,
                             hist_size,
                             ranges);
        }
        Mat::new_with_cmat(m)
    }

    pub fn calc_back_project(&self,
                             channels: *const i32,
                             hist: &Mat,
                             ranges: *const *const f32)
                             -> Mat {
        let m = unsafe { opencv_mat_new() };
        unsafe {
            opencv_calc_back_project(self.c_mat,
                                     1,
                                     channels,
                                     (*hist).c_mat,
                                     m,
                                     ranges);
        }
        Mat::new_with_cmat(m)
    }
}

// =============================================================================
//   Highgui: high-level GUI
// =============================================================================
extern "C" {
    pub fn opencv_named_window(name: *const c_char, flags: c_int);
    fn opencv_imshow(name: *const c_char, cmat: *mut CMat);
    fn opencv_wait_key(delay_ms: c_int) -> c_int;
    pub fn opencv_set_mouse_callback(name: *const c_char,
                                     on_mouse: extern "C" fn(e: i32,
                                                             x: i32,
                                                             y: i32,
                                                             f: i32,
                                                             data: *mut c_void),
                                     userdata: *mut c_void);
}

pub enum WindowFlags {
    WindowNormal = 0x00000000,
    WindowAutosize = 0x00000001,
    WindowOpengl = 0x00001000,
}

/// Mouse Events
pub enum MouseEventTypes {
    /// Indicates that the mouse has moved over the window.
    MouseMove = 0,
    /// Indicates that the left mouse button is pressed.
    LButtonDown = 1,
    /// Indicates that the right mouse button is pressed.
    RButtonDown = 2,
    /// Indicates that the middle mouse button is pressed.
    MButtonDown = 3,
    /// Indicates that left mouse button is released.
    LButtonUp = 4,
    /// Indicates that right mouse button is released.
    RButtonUp = 5,
    /// Indicates that middle mouse button is released.
    MButtonUp = 6,
    /// Indicates that left mouse button is double clicked.
    LButtonClick = 7,
    /// Indicates that right mouse button is double clicked.
    RButtonClick = 8,
    /// Indicates that middle mouse button is double clicked.
    MButtonClick = 9,
    /// Positive/negative means forward/backward scrolling.
    MouseWheel = 10,
    /// Positive/negative means right and left scrolling.
    MouseHWheel = 11,
}

// =============================================================================
//   VideoCapture
// =============================================================================
type CVideoCapture = c_void;
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

type CCascadeClassifier = c_void;
pub struct CascadeClassifier {
    c_cascade_classifier: *mut CCascadeClassifier,
}

extern "C" {
    fn opencv_cascade_classifier_new() -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_from_path(p: *const c_char)
                                           -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_drop(p: *mut CCascadeClassifier);
    fn opencv_cascade_classifier_detect(cc: *mut CCascadeClassifier,
                                        cmat: *mut CMat,
                                        vec_of_rect: *mut CVecOfRect);
}

impl CascadeClassifier {
    pub fn new() -> Self {
        let cascade = unsafe { opencv_cascade_classifier_new() };
        CascadeClassifier { c_cascade_classifier: cascade }
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
                                             result.get_mut_c_vec_of_rec());
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
type CTermCriteria = c_void;
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

impl RotatedRect {
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
