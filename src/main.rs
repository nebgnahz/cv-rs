extern crate rust_vision;
use rust_vision::*;
use std::ffi::CString;

fn help() {
    println!(r#"Usage:
cargo run -- <image_path>
"#);
    std::process::exit(0);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        help();
    }

    let x = Mat::new(&args[1], 1);
    let s = CString::new("Window").unwrap();
    unsafe {
        opencv_named_window((&s).as_ptr(), WindowFlags::WindowAutosize as i32);
    }

    x.show("window", 1000);
    println!("Hello, OpenCV with Rust!");
}
