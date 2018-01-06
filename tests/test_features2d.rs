extern crate cv;
mod utils;

use utils::*;
use cv::mser::*;

#[test]
fn mser_lenna() {
    let lenna = load_lenna();
    let mser: MSER = MSERBuilder::new().into();
    let (msers, boxes) = mser.detect_regions(&lenna);
    assert_ne!(msers.len(), 0);
    assert_ne!(boxes.len(), 0);
}
