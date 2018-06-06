extern crate cv;
extern crate float_cmp;
mod floatutils;
mod utils;

use cv::imgcodecs::ImageReadMode;
use cv::phash::*;
use cv::*;
use floatutils::*;
use utils::*;

#[test]
fn phash_test() {
    let image_path = get_asset_path("Ubuntu.png");
    let image_path2 = get_asset_path("lenna.png");
    let mat = Mat::from_path(image_path, ImageReadMode::Grayscale).unwrap();
    let lenna = Mat::from_path(image_path2, ImageReadMode::Grayscale).unwrap();
    let phash = PHash::new();
    let hash = phash.compute(&mat);
    let hash2 = phash.compute(&mat);
    let lenna_hash = phash.compute(&lenna);
    let diff = phash.compare(&hash, &hash2);
    let diff_lenna = phash.compare(&hash, &lenna_hash);

    assert_eq(diff, 0.0);
    assert_ne(diff_lenna, 0.0);
}
