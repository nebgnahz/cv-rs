#![feature(test)]
extern crate test;
extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::ObjectDetect;

mod utils;
use utils::*;

#[test]
fn detect_lenna() {
    let mat = load_lenna();
    let cascade = load_frontal_face();
    let result = cascade.detect(&mat);
    assert_eq!(result.len(), 1);
    assert!(close_rect(result[0].0,
                       Rect {
                           x: 219,
                           y: 203,
                           width: 170,
                           height: 170,
                       },
                       2));
}
