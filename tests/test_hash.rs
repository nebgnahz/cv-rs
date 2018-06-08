extern crate cv;
extern crate float_cmp;
mod floatutils;
mod utils;

use cv::hash::*;
use cv::imgcodecs::ImageReadMode;
use cv::*;
use floatutils::*;
use utils::*;

#[test]
fn phash_test() {
    test(PHash::new(), 30.0);
}

fn test<T: Hash>(hash: T, expected_diff: f64) {
    let lenna = get_asset_path("lenna.png");
    let solvay_conference = get_asset_path("Solvay_conference_1927.jpg");
    let lenna = Mat::from_path(lenna, ImageReadMode::Grayscale).unwrap();
    let solvay_conference = Mat::from_path(solvay_conference, ImageReadMode::Grayscale).unwrap();
    let lenna_hash = hash.compute(&lenna);
    let solvay_hash = hash.compute(&solvay_conference);
    let diff = hash.compare(&lenna_hash, &solvay_hash);
    assert_eq(diff, expected_diff)
}
