extern crate rust_vision;
use rust_vision::*;
use std::ffi::CString;

const CASCADE_PATH: &'static str = "/usr/local/Cellar/opencv3/3.1.0_3/share/OpenCV/haarcascades/haarcascade_frontalface_default.xml";

fn main() {
    let cap = VideoCapture::new(0);
    assert!(cap.is_open());
    let mut m = Mat::new();

    let cascade = CascadeClassifier::from_path(CASCADE_PATH);
    let s = CString::new("Window").unwrap();
    unsafe {
        opencv_named_window((&s).as_ptr(), WindowFlags::WindowAutosize as i32);
    }

    loop {
        let mut result = VecOfRect::default();
        cap.read(&m);
        cascade.detect(&m, &mut result);
        result.draw_on_mat(&mut m);
        m.show("window", 30);
    }
}
