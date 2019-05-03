//! The test suite in this file is adapted from:
//! https://docs.opencv.org/3.1.0/d3/df2/tutorial_py_basic_ops.html
//!
//! N.B. Blue pixel value at (100, 100) is actually 156 on my laptop (Mac OS),
//! but 157 on Travis (trusty). This nuance comes from JPEG decoding...

extern crate cv;
mod utils;

use cv::*;
use cv::mat::Merge;

#[test]
fn test_accessing_pixel() {
    let img = utils::load_messi_color();

    let pixel = img.at2::<(u8, u8, u8)>(100, 100);
    assert!(pixel_eq(pixel.0, 156));
    assert!(pixel_eq(pixel.1, 166));
    assert!(pixel_eq(pixel.2, 200));

    let blue = img.at3::<u8>(100, 100, 0);
    assert!(pixel_eq(blue, 156));
}

#[test]
fn test_mat_type() {
    let img = utils::load_lenna();
    let res = img.cv_type();
    assert_eq!(res, CvType::Cv8UC1);
}

#[test]
fn test_mat_clone() {
    let img = utils::load_lenna();
    let img2 = img.clone();
    assert_eq!(img.rows, img2.rows);
    assert_eq!(img.cols, img2.cols);
}

fn pixel_eq(a: u8, b: u8) -> bool {
    (a - b) <= 1
}

#[test]
fn test_mat_merge() {
    let size = Size2i::new(100, 200);
    let img = utils::load_lenna().resize_to(size.clone(), cv::imgproc::InterpolationFlag::InterNearst);
    let img2 = utils::load_messi_color().resize_to(size.clone(), cv::imgproc::InterpolationFlag::InterNearst);
    let merged = vec![&img, &img2].merge();
    assert_eq!(merged.channels, img.channels + img2.channels);
    assert_eq!(merged.size().width, size.width);
    assert_eq!(merged.size().height, size.height);
}
