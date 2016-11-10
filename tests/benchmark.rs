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
fn bench_face_detect_physicists() {
    let mat = load_physicists();
    let cascade = load_frontal_face();

    (0..10).map(|i| {
        let rate = 1.0 - (i as f64) * 0.1;
        let smaller = mat.resize_by(rate, rate, InterpolationFlag::InterLinear);
        let bench_name = format!("detect physicists: {}x{}", smaller.rows, smaller.cols);
        timed_multiple(&bench_name, 1, || {
            cascade.detect(&smaller);
        });
    }).collect::<Vec<_>>();
}
