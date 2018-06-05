extern crate cv;
mod utils;

use cv::imgcodecs::ImageReadMode;
use cv::*;
use cv::phash::*;
use utils::*;

#[test]
fn phash_test() {
    let image_path = get_asset_path("Ubuntu.png");
    let mat = Mat::from_path(image_path, ImageReadMode::Grayscale).unwrap();
    let phash = PHash::new();
    let hash = phash.compute(&mat);

    assert_ne!(hash.cols, 0);
}
