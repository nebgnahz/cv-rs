extern crate cv;
mod utils;

use cv::*;
use cv::imgcodecs::ImreadModes;
use cv::text::*;
use std::path::PathBuf;
use utils::*;

const VOCABULARY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[test]
#[cfg(feature = "tesseract")]
fn ocr_tesseract_test_line() {
    let image = Mat::from_path("assets/HelloWorld.png", ImreadModes::ImreadColor).unwrap();
    let path = PathBuf::from("/usr/share/tesseract-ocr");
    let ocr = OcrTesseract::new(
        Some(&path),
        Some("eng"),
        Some(VOCABULARY),
        EngineMode::Default,
        PageSegmentationMode::Auto,
    ).unwrap();
    let res = ocr.run(&image, ComponentLevel::TextLine);
    assert_contains(&res.0, "Heruro worudo")
}

#[test]
#[cfg(feature = "tesseract")]
fn ocr_tesseract_test_word() {
    let image = Mat::from_path("assets/Ubuntu.png", ImreadModes::ImreadColor).unwrap();
    let path = PathBuf::from("/usr/share/tesseract-ocr");
    let ocr = OcrTesseract::new(
        Some(&path),
        Some("eng"),
        Some(VOCABULARY),
        EngineMode::Default,
        PageSegmentationMode::Auto,
    ).unwrap();
    let res = ocr.run(&image, ComponentLevel::Word);
    assert_contains(&res.0, "uBuntu")
}

#[test]
fn ocr_hmm_test() {
    let image = Mat::from_path("assets/Ubuntu.png", ImreadModes::ImreadGrayscale).unwrap();
    let classifier_name = get_asset_path("OCRHMM_knn_model_data.xml.gz");
    let transition_probability_path = PathBuf::from("assets/OCRHMM_transitions_table.xml");
    let transition_probability_table =
        Mat::from_file_storage(&transition_probability_path, "transition_probabilities").unwrap();
    let emission_probability_table = Mat::eye(
        VOCABULARY.len() as i32,
        VOCABULARY.len() as i32,
        CvType::Cv64FC1,
    );
    let ocr = OcrHmmDecoder::new(
        &classifier_name,
        VOCABULARY,
        &transition_probability_table,
        &emission_probability_table,
        ClassifierType::Knn,
    ).unwrap();
    let res = ocr.run(&image, ComponentLevel::Word);
    let reslen = res.0.len();
    assert_ne!(reslen, 0); // do not check actual recognized text, waiting for fix: https://github.com/opencv/opencv_contrib/issues/1557
}

#[cfg(feature = "tesseract")]
fn assert_contains(left: &str, right: &str) {
    assert!(left.contains(right), "{} != {}", left, right);
}
