extern crate libc;
use libc::{c_int, c_void};

use std::os::raw::c_char;

extern "C" {
    pub fn opencv_imread(input: *const c_char, flags: c_int) -> c_void;
}
