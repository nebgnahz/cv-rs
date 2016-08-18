extern crate libc;
use libc::{c_int, c_void};
use std::os::raw::c_char;

pub type Mat = c_void;

extern "C" {
    pub fn opencv_imread(input: *const c_char, flags: c_int) -> *mut Mat;
    pub fn opencv_mat_free(mat: *mut Mat);
}
