extern crate cv;
mod utils;

use cv::*;
use cv::imgcodecs::ImreadModes;
use cv::text::*;

#[test]
fn ocr_tesseract_test() {
    let image = Mat::from_path("assets/HelloWorld.png", ImreadModes::ImreadColor).unwrap();
    let ocr = OcrTesseract::new(
        None,
        None,
        None,
        EngineMode::Default,
        PageSegmentationMode::Auto,
    );
    let res = ocr.run(&image, ComponentLevel::TextLine);
    assert_eq!(res.0, "Heruro worudo");
}
