#![allow(dead_code)]

extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

// TODO(benzh): For some reason, Cargo keeps saying these functions are not
// used. I guess each individual file in `tests` are compiled, so this file is
// compiled in its own, making these functions appearing useless.

pub fn close_rect(a: Rect, b: Rect, epsilon: i32) -> bool {
    ((a.x - b.x) < epsilon) && ((a.y - b.y) < epsilon) && ((a.width - b.width)) < epsilon &&
    ((a.height - b.height)) < epsilon
}

pub fn timed<F>(label: &str, inner: F)
    where F: FnMut()
{
    timed_multiple(label, 1, inner);
}

pub fn timed_multiple<F>(label: &str, iteration: usize, mut inner: F)
    where F: FnMut()
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

pub fn load_lenna() -> Mat {
    let buf = load_lenna_as_buf();
    Mat::imdecode(&buf, ImreadModes::ImreadGrayscale)
}

pub fn load_lenna_as_buf() -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/lenna.png");
    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    buf
}

pub fn load_frontal_face() -> CascadeClassifier {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    CascadeClassifier::from_path(d)
}
