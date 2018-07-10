extern crate cv;

use cv::highgui::*;
use cv::imgcodecs::*;
use cv::objdetect::CascadeClassifier;
use cv::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/Solvay_conference_1927.jpg");

    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::image_decode(&buf, ImageReadMode::Grayscale);

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    let cascade = CascadeClassifier::from_path(d).unwrap();

    highgui_named_window("window", WindowFlag::Normal).unwrap();

    // result is a vector of rectangles
    let result = cascade.detect_with_params(&mat, 1.1, 15, Size2i::new(80, 80), Size2i::default());

    println!("Detected {} faces", result.len());
    // we draw each of them on the image
    result
        .iter()
        .map(|&r| mat.rectangle_custom(r.scale(1.2), Scalar::new(255, 255, 0, 255), 10, LineType::Line8))
        .count();
    mat.show("window", 0).unwrap();
}
