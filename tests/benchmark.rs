extern crate cv;

use cv::imgcodecs::*;
use cv::imgproc::*;
use cv::objdetect::ObjectDetect;
use cv::*;
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
        Mat::image_decode(&buf, ImageReadMode::Grayscale);
    });
}

#[test]
fn bench_face_detect_physicists() {
    let mat = load_physicists();
    let cascade = load_frontal_face();

    for i in 0..3 {
        let rate = 1.0 - (i as f64) * 0.1;
        let m = mat.resize_by(rate, rate, InterpolationFlag::InterLinear);
        let name = format!("detect physicists: {}x{}", m.rows, m.cols);
        timed_multiple(&name, 1, || {
            cascade.detect(&m);
        });
    }
}
