extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/test.jpg");

    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let mut hog = HogDescriptor::new();
    let detector = SvmDetector::default_people_detector();
    hog.set_svm_detector(detector);

    let start = ::std::time::Instant::now();
    let results = hog.detect(&mat, Size2i::new(1, 1), Size2i::new(1, 1), 1.1);

    let elapsed = start.elapsed();
    println!("{} ms",
             elapsed.as_secs() as f64 * 1_000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0);

    highgui_named_window("window", WindowFlags::WindowAutosize);

    // we draw each of them on the image
    results.iter()
        .map(|&(r, _w)| mat.rectangle(r))
        .count();
    mat.show("window", 0);
}
