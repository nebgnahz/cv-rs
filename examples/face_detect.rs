extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::{ObjectDetect, CascadeClassifier};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/lenna.png");

    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    let cascade = CascadeClassifier::from_path(d);

    highgui_named_window("window", WindowFlags::WindowAutosize);

    // result is a vector of rectangles
    let result = cascade.detect(&mat);
    println!("result: {:?}", result);
    // we draw each of them on the image
    result.iter().map(|&(r, _conf)| mat.rectangle(r)).count();
    mat.show("window", 0);
}
