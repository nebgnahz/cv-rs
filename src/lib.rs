extern crate libc;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::os::raw::c_char;

type CMat = c_void;

pub struct Mat {
    c_mat: *mut CMat,
}

extern "C" {
    fn opencv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn opencv_mat_free(mat: *mut CMat);
}

impl Mat {
    pub fn new(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { opencv_imread((&s).as_ptr(), flags) };
        Mat {
            c_mat: m,
        }
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            opencv_mat_free(self.c_mat);
        }
    }
}
