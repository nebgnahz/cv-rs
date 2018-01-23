extern crate cv;
mod utils;

use cv::features2d::*;
use utils::*;

#[test]
fn mser_lenna() {
    let lenna = load_lenna();
    let mser: MSER = MSERBuilder::default().into();
    let (msers, boxes) = mser.detect_regions(&lenna);
    assert_ne!(msers.len(), 0);
    assert_ne!(boxes.len(), 0);
}
