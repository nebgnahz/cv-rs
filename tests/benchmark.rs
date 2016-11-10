extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::ObjectDetect;
mod utils;
use utils::*;

#[test]
fn bench_mat_new() {
    timed("create new mat", || {
        Mat::new();
    });
}

#[test]
fn bench_decode_lenna() {
    let buf = load_lenna_as_buf();
    timed("decode lenna.png", || {
        Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);
    });
}

#[test]
fn bench_face_detect_lenna() {
    let mat = load_lenna();
    let cascade = load_frontal_face();

    timed_multiple("face detecting lenna", 5, || {
        cascade.detect(&mat);
    });
}
