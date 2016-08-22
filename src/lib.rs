//! A Rust wrapper for OpenCV.

extern crate libc;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::os::raw::c_char;

type CMat = c_void;
pub struct Mat {
    c_mat: *mut CMat,
}

#[repr(C)]
pub struct Scalar {
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
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
    fn opencv_mat_is_valid(mat: *mut CMat) -> bool;
    fn opencv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn opencv_mat_roi(cmat: *const CMat, rect: Rect) -> *mut CMat;
    fn opencv_mat_drop(mat: *mut CMat);
}

impl Mat {
    pub fn new() -> Self {
        let m = unsafe { opencv_mat_new() };
        Mat { c_mat: m }
    }

    pub fn from_path(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { opencv_imread((&s).as_ptr(), flags) };
        Mat { c_mat: m }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { opencv_mat_is_valid(self.c_mat) }
    }

    pub fn roi(&self, rect: Rect) -> Mat {
        let cmat = unsafe {
            opencv_mat_roi(self.c_mat, rect)
        };
        Mat {
            c_mat: cmat,
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
//  Operations on arrays
// =============================================================================
extern "C" {
    fn opencv_in_range(cmat: *const CMat, lowerb: Scalar, upperb: Scalar, dst: *mut CMat);
}

impl Mat {
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = Mat::new();
        unsafe { opencv_in_range(self.c_mat, lowerb, upperb, m.c_mat) }
        m
    }
}

extern "C" {
    fn opencv_rectangle(cmat: *mut CMat, rect: Rect);
    fn opencv_cvt_color(cmat: *const CMat, output: *mut CMat, code: i32);
}

impl Mat {
    pub fn rectangle(&self, rect: Rect) {
        unsafe {
            opencv_rectangle(self.c_mat, rect);
        }
    }

    pub fn cvt_color(&self, code: i32) -> Mat {
        let m = Mat::new();
        unsafe { opencv_cvt_color(self.c_mat, m.c_mat, code) }
        m
    }
}

extern "C" {
    pub fn opencv_named_window(name: *const c_char, flags: c_int);
    fn opencv_imshow(name: *const c_char, cmat: *mut CMat);
    fn opencv_wait_key(delay_ms: c_int) -> c_int;
}

pub enum WindowFlags {
    WindowNormal = 0x00000000,
    WindowAutosize = 0x00000001,
    WindowOpengl = 0x00001000,
}

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
