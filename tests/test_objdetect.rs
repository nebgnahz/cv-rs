#![feature(test)]
extern crate test;
extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::CascadeClassifier;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn detect_lenna() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/lenna.png");

    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    let cascade = CascadeClassifier::from_path(d);

    // result is a vector of rectangles
    let result = cascade.detect(&mat);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0],
               Rect {
                   x: 219,
                   y: 203,
                   width: 170,
                   height: 170,
               });
}
