extern crate cv;
mod utils;

use cv::text::*;
use std::path::PathBuf;
use utils::load_lenna;

#[test]
fn ocr_tesseract_test() {
    let lenna = load_lenna();
    let path = PathBuf::new();
    let ocr = OcrTesseract::new(
        Some(&path),
        None,
        None,
        EngineMode::Default,
        PageSegmentationMode::Auto,
    );
    let res = ocr.run(&lenna, ComponentLevel::Word);
    assert_ne!(res.1.len(), 0);
}
