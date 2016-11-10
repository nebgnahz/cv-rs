extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::*;
use rust_vision::objdetect::HogDescriptor as Hog;

fn main() {
    // Video IO
    let video_path = ::std::env::args().nth(1).unwrap();
    let cap = VideoCapture::from_path(&video_path);
    let m = Mat::new();
    highgui_named_window("Window", WindowFlags::WindowAutosize);

    // HOG
    let mut params = HogParams::default();
    params.scale = 1.01;
    params.padding = Size2i::new(16, 16);
    let mut hog = Hog::with_params(params);
    println!("{:?}", hog.params);
    let detector = SvmDetector::default_people_detector();
    hog.set_svm_detector(detector);

    let mut frame_num = 0;
    loop {
        cap.read(&m);

        let start = ::std::time::Instant::now();
        let results = hog.detect(&m);
        let elapsed = start.elapsed();
        println!("{},{},{}",
                 frame_num,
                 results.len(),
                 duration_as_ms(elapsed));

        results.iter()
            .map(|&(r, _w)| m.rectangle(r.scale(0.6)))
            .count();
        m.show("Window", 1);
        frame_num += 1;
    }
}

fn duration_as_ms(d: ::std::time::Duration) -> f64 {
    d.as_secs() as f64 * 1_000.0 + d.subsec_nanos() as f64 / 1_000_000.0
}
