// TODO(benzh): For some reason, Cargo keeps saying these functions are not
// used. I guess each individual file in `tests` are compiled, so this file is
// compiled in its own, making these functions appearing useless.
#![allow(dead_code)]

extern crate cv;

use cv::imgcodecs::*;
use cv::objdetect::*;
use cv::*;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn close_rect(a: Rect, b: Rect, epsilon: i32) -> bool {
    ((a.x - b.x) < epsilon) && ((a.y - b.y) < epsilon) && (a.width - b.width) < epsilon
        && (a.height - b.height) < epsilon
}

pub fn timed<F>(label: &str, inner: F)
where
    F: FnMut(),
{
    timed_multiple(label, 1, inner);
}

pub fn timed_multiple<F>(label: &str, iteration: usize, mut inner: F)
where
    F: FnMut(),
{
    let total: f64 = (0..iteration)
        .map(|_| {
            let start = Instant::now();
            inner();
            let elapsed = start.elapsed();
            elapsed.as_secs() as f64 * 1_000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0
        })
        .sum();
    println!("  {}: {} ms", label, total / (iteration as f64));
}

pub fn load_physicists() -> Mat {
    let buf = load_image_as_buf("assets/Solvay_conference_1927.jpg");
    Mat::image_decode(&buf, ImageReadMode::Grayscale)
}

pub fn load_avg_towncentre() -> Mat {
    let buf = load_image_as_buf("assets/AVG-TownCentre-test-000011.jpg");
    Mat::image_decode(&buf, ImageReadMode::Grayscale)
}

pub fn load_lenna() -> Mat {
    let buf = load_lenna_as_buf();
    Mat::image_decode(&buf, ImageReadMode::Grayscale)
}

pub fn load_messi_color() -> Mat {
    let buf = load_image_as_buf("assets/messi5.jpg");
    Mat::image_decode(&buf, ImageReadMode::Color)
}

pub fn load_lenna_as_buf() -> Vec<u8> {
    load_image_as_buf("assets/lenna.png")
}

fn load_image_as_buf<P: AsRef<Path>>(img: P) -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(img);
    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    buf
}

pub fn load_frontal_face() -> CascadeClassifier {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    CascadeClassifier::from_path(d).unwrap()
}

pub fn get_asset_path(name: &'static str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join(name)
}
