extern crate cv;
mod utils;

use cv::*;
use cv::imgcodecs::ImreadModes;
use cv::text::*;
use std::path::PathBuf;

#[test]
fn ocr_tesseract_test() {
    let image = Mat::from_path("assets/HelloWorld.png", ImreadModes::ImreadColor).unwrap();
    let path = PathBuf::from("/usr/share/tesseract-ocr");
    let vec = vec!['z' as std::os::raw::c_char];
    let ocr = OcrTesseract::new(
        Some(&path),
        Some("eng"),
        Some(&vec),
        EngineMode::Default,
        PageSegmentationMode::Auto,
    );
    let res = ocr.run(&image, ComponentLevel::TextLine);
    assert!(res.0.contains("Heruro worudo"));
}
