#![cfg(feature = "text")]
extern crate cv;
mod utils;

use cv::imgcodecs::ImageReadMode;
use cv::text::*;
use cv::*;
use utils::*;

const VOCABULARY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[cfg(feature = "tesseract")]
mod tesseract {
    use super::*;
    use std::path::Path;

    #[test]
    fn ocr_tesseract_test_line() {
        let image_path = get_asset_path("HelloWorld.png");
        let image = Mat::from_path(image_path, ImageReadMode::Color).unwrap();
        let path = Path::new("/usr/share/tesseract-ocr");
        let ocr = OcrTesseract::new(
            Some(&path),
            Some("eng"),
            Some(VOCABULARY),
            EngineMode::Default,
            PageSegmentationMode::Auto,
        )
        .unwrap();
        let res = ocr.run(&image, ComponentLevel::TextLine);
        assert_contains(&res.0, "Heruro worudo")
    }

    #[test]
    fn ocr_tesseract_test_word() {
        let image_path = get_asset_path("Ubuntu.png");
        let image = Mat::from_path(&image_path, ImageReadMode::Color).unwrap();
        let path = Path::new("/usr/share/tesseract-ocr");
        let ocr = OcrTesseract::new(
            Some(&path),
            Some("eng"),
            Some(VOCABULARY),
            EngineMode::Default,
            PageSegmentationMode::Auto,
        )
        .unwrap();
        let res = ocr.run(&image, ComponentLevel::Word);
        assert_contains(&res.0, "uBuntu")
    }

    fn assert_contains(left: &str, right: &str) {
        assert!(left.contains(right), "{} != {}", left, right);
    }
}

#[test]
#[ignore]
fn ocr_hmm_test() {
    let image_path = get_asset_path("Ubuntu.png");
    let classifier_name = get_asset_path("OCRHMM_knn_model_data.xml.gz");
    let transition_probability_path = get_asset_path("OCRHMM_transitions_table.xml");

    let image = Mat::from_path(&image_path, ImageReadMode::Grayscale).unwrap();
    let transition_probability_table =
        Mat::from_file_storage(&transition_probability_path, "transition_probabilities").unwrap();
    let emission_probability_table = Mat::eye(VOCABULARY.len() as i32, VOCABULARY.len() as i32, CvType::Cv64FC1);
    let ocr = OcrHmmDecoder::new(
        &classifier_name,
        VOCABULARY,
        &transition_probability_table,
        &emission_probability_table,
        ClassifierType::Knn,
    )
    .unwrap();
    let res = ocr.run(&image, ComponentLevel::Word);
    let reslen = res.0.len();

    assert_ne!(reslen, 0); // do not check actual recognized text, waiting for fix: https://github.com/opencv/opencv_contrib/issues/1557
}

#[test]
#[should_panic]
fn ocr_holistic_word_panic() {
    let _ = OcrHolisticWord::new("a", "a", "a").unwrap();
}
