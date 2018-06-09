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
fn average_hash_test() {
    test(AverageHash::new(), 30.0);
}

#[test]
fn block_mean_hash_test() {
    test(BlockMeanHash::new(), 113.0);
}

#[test]
fn color_moment_hash_test() {
    test(ColorMomentHash::new(), 22.5625);
}

#[test]
fn marr_hildreth_hash_test() {
    test(MarrHildrethHash::new(), 307.0);
}

#[test]
fn phash_test() {
    test(PHash::new(), 30.0);
}

#[test]
fn radial_variance_hash_test() {
    test(RadialVarianceHash::new(), 0.30779);
}

fn test<T: Hash>(hash: T, expected_diff: f64) {
    let lenna = get_asset_path("lenna.png");
    let solvay_conference = get_asset_path("Solvay_conference_1927.jpg");
    let lenna = Mat::from_path(lenna, ImageReadMode::Color).unwrap();
    let solvay_conference = Mat::from_path(solvay_conference, ImageReadMode::Color).unwrap();
    let lenna_hash = hash.compute(&lenna);
    let solvay_hash = hash.compute(&solvay_conference);
    let diff = hash.compare(&lenna_hash, &solvay_hash);
    assert_eq(diff, expected_diff)
}
