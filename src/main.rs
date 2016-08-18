extern crate rust_vision;

use rust_vision::opencv_imread;
use std::ffi::CString;

fn main() {
    let s = CString::new("placeholder").unwrap();
    let _ = unsafe { opencv_imread((&s).as_ptr(), 1) };
    println!("Hello, OpenCV with Rust!");
}
