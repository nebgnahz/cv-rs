extern crate libc;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::os::raw::c_char;

type CMat = c_void;

pub struct Mat {
    c_mat: *mut CMat,
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
    fn opencv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    fn opencv_mat_free(mat: *mut CMat);
    pub fn opencv_named_window(name: *const c_char, flags: c_int);
    fn opencv_imshow(name: *const c_char, cmat: *mut CMat);
    fn opencv_wait_key(delay_ms: c_int) -> c_int;
}

impl Mat {
    pub fn new(path: &str, flags: i32) -> Self {
        let s = CString::new(path).unwrap();
        let m = unsafe { opencv_imread((&s).as_ptr(), flags) };
        Mat {
            c_mat: m,
        }
    }

    pub fn show(self, name: &str, delay: i32) {
        let s = CString::new(name).unwrap();
        unsafe {
            opencv_imshow((&s).as_ptr(), self.c_mat);
            opencv_wait_key(delay);
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
