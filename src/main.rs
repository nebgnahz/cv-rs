extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::CascadeClassifier;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/Solvay_conference_1927.jpg");

    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    let cascade = CascadeClassifier::from_path(d);

    highgui_named_window("window", WindowFlags::WindowAutosize);
    let result = cascade.detect_with_params(&mat, 1.1, 15, Size2i::new(80, 80), Size2i::default());
    println!("result: {:?}", result);
    result.iter()
        .map(|r| {
            println!("{:?}", r);
            mat.rectangle(*r)
        })
        .collect::<Vec<_>>();
    mat.show("window", 0);
}
