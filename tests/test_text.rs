extern crate cv;
mod utils;

use cv::*;
use cv::imgcodecs::ImreadModes;
use cv::imgproc::*;
use cv::text::*;
use std::path::PathBuf;

#[test]
fn ocr_tesseract_test_line() {
    let image = Mat::from_path("assets/HelloWorld.png", ImreadModes::ImreadColor).unwrap();
    let path = PathBuf::from("/usr/share/tesseract-ocr");
    let ocr = OcrTesseract::new(
        Some(&path),
        Some("eng"),
        None,
        EngineMode::Default,
        PageSegmentationMode::Auto,
    );
    let res = ocr.run(&image, ComponentLevel::TextLine);
    assert_contains(&res.0, "Heruro worudo")
}

#[test]
fn ocr_tesseract_test_word() {
    let image = Mat::from_path("assets/Ubuntu.png", ImreadModes::ImreadColor).unwrap();
    let path = PathBuf::from("/usr/share/tesseract-ocr");
    let ocr = OcrTesseract::new(
        Some(&path),
        Some("eng"),
        None,
        EngineMode::Default,
        PageSegmentationMode::Auto,
    );
    let res = ocr.run(&image, ComponentLevel::Word);
    assert_contains(&res.0, "uBuntu")
}

#[test]
fn ocr_hmm_test() {
    let image = Mat::from_path("assets/Ubuntu.png", ImreadModes::ImreadColor).unwrap();
    let image = image.cvt_color(ColorConversionCodes::BGR2GRAY);
    let classifier_name = PathBuf::from("assets/OCRHMM_knn_model_data.xml.gz");
    let transition_probability_path = PathBuf::from("assets/OCRHMM_transitions_table.xml");
    let transition_probability_table =
        Mat::from_file_storage(&transition_probability_path, "transition_probabilities").unwrap();
    let emission_probability_table = Mat::eye(62, 62, CvType::Cv32SC1);
    let ocr = OcrHmmDecoder::new(
        &classifier_name,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        &transition_probability_table,
        &emission_probability_table,
    ).unwrap();
    let res = ocr.run(&image, ComponentLevel::Word);
    assert_contains(&res.0, "uBuntu");
}

fn assert_contains(left: &str, right: &str) {
    assert!(left.contains(right), "{} != {}", left, right);
}
