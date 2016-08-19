extern crate libc;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::os::raw::c_char;

type CMat = c_void;
pub struct Mat {
    c_mat: *mut CMat,
}

#[repr(C)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

type CVideoCapture = c_void;
pub struct VideoCapture {
    c_videocapture: *mut CVideoCapture,
}

type CCascadeClassifier = c_void;
pub struct CascadeClassifier {
    c_cascade_classifier: *mut CCascadeClassifier,
}

extern "C" {
    fn opencv_mat_new() -> *mut CMat;
    fn opencv_mat_is_valid(mat: *mut CMat) -> bool;
    fn opencv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn opencv_mat_drop(mat: *mut CMat);
}

impl Mat {
    pub fn new() -> Self {
        let m = unsafe { opencv_mat_new() };
        Mat {
            c_mat: m,
        }
    }

    pub fn from_path(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { opencv_imread((&s).as_ptr(), flags) };
        Mat {
            c_mat: m,
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { opencv_mat_is_valid(self.c_mat) }
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

extern "C" {
    pub fn opencv_named_window(name: *const c_char, flags: c_int);
    fn opencv_imshow(name: *const c_char, cmat: *mut CMat);
    fn opencv_wait_key(delay_ms: c_int) -> c_int;
}

pub enum WindowFlags {
    WindowNormal       = 0x00000000,
    WindowAutosize     = 0x00000001,
    WindowOpengl       = 0x00001000,

    // The following flags are weird: FULLSCREEN is the same as AUTOSIZE
    // Disabling them for now!
    // WINDOW_FULLSCREEN   = 1,
    // WINDOW_FREERATIO    = 0x00000100,
    // WINDOW_KEEPRATIO    = 0x00000000,
    // WINDOW_GUI_EXPANDED = 0x00000000,
    // WINDOW_GUI_NORMAL   = 0x00000010,
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
        VideoCapture {
            c_videocapture: cap,
        }
    }

    pub fn is_open(&self, ) -> bool {
        unsafe {
            opencv_videocapture_is_opened(self.c_videocapture)
        }
    }

    pub fn read(&self, mat: &Mat) -> bool {
        unsafe {
            opencv_videocapture_read(self.c_videocapture, mat.get_cmat())
        }
    }
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        unsafe {
            opencv_videocapture_drop(self.c_videocapture);
        }
    }
}

extern "C" {
    fn opencv_cascade_classifier_new() -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_from_path(p: *const c_char) -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_drop(p: *mut CCascadeClassifier);
}

impl CascadeClassifier {
    pub fn new() -> Self {
        let cascade = unsafe {
            opencv_cascade_classifier_new()
        };
        CascadeClassifier {
            c_cascade_classifier: cascade,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let s = CString::new(path).unwrap();
        let cascade = unsafe {
            opencv_cascade_classifier_from_path((&s).as_ptr())
        };
        CascadeClassifier {
            c_cascade_classifier: cascade,
        }
    }
}

impl Drop for CascadeClassifier {
    fn drop(&mut self) {
        unsafe {
            opencv_cascade_classifier_drop(self.c_cascade_classifier);
        }
    }
}
