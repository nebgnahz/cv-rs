extern crate rust_vision;
use rust_vision::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: face_detect CascadePath");
        std::process::exit(-1);
    }

    let cap = VideoCapture::new(0);
    assert!(cap.is_open());
    let mut m = Mat::new();

    let cascade = CascadeClassifier::from_path(&args[1]);
    highgui_named_window("Window", WindowFlags::WindowAutosize);

    loop {
        let mut result = VecOfRect::default();
        cap.read(&m);
        cascade.detect(&m, &mut result);
        result.draw_on_mat(&mut m);
        m.show("window", 30);
    }
}
