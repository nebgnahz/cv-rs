extern crate rust_vision;

use rust_vision::*;
use std::ffi::CString;

fn main() {
    let s = CString::new("placeholder").unwrap();
    let m = unsafe { opencv_imread((&s).as_ptr(), 1) };
    unsafe { opencv_mat_free(m) };
    println!("Hello, OpenCV with Rust!");
}
