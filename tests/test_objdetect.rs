extern crate cv;

use cv::*;
use cv::objdetect::ObjectDetect;

mod utils;
use utils::*;

#[test]
fn detect_lenna() {
    let mat = load_lenna();
    let cascade = load_frontal_face();
    let result = cascade.detect(&mat);
    assert_eq!(result.len(), 1);
    assert!(close_rect(
        result[0].0,
        Rect {
            x: 220,
            y: 204,
            width: 168,
            height: 168,
        },
        3,
    ));
}
