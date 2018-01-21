/// These tests will run regardless of cuda or not. When tested with `--features
/// cuda`, it will use CUDA-enabled `HOG` and `CascadeClassifier`.
extern crate cv;

#[cfg(feature = "gpu")]
use cv::cuda::GpuHog as Hog;
#[cfg(not(feature = "gpu"))]
use cv::objdetect::HogDescriptor as Hog;

#[cfg(feature = "gpu")]
use cv::cuda::GpuCascade as CascadeClassifier;
#[cfg(not(feature = "gpu"))]
use cv::objdetect::CascadeClassifier;

use cv::objdetect::SvmDetector;
use cv::objdetect::HogParams;
use cv::objdetect::ObjectDetect;
mod utils;

#[test]
fn test_pedestrian_detection() {
    let mat = utils::load_avg_towncentre();

    let mut params = HogParams::default();
    params.hit_threshold = 0.3;
    let mut hog = Hog::with_params(params);
    let detector = SvmDetector::default_people_detector();
    hog.set_svm_detector(detector);
    let result = hog.detect(&mat);
    assert!(result.len() > 1);
}

#[test]
fn test_cascade_lenna() {
    let mat = utils::load_lenna();
    let model_path = cascade_model_path();
    let cascade = CascadeClassifier::from_path(model_path).unwrap();
    let result = cascade.detect(&mat);
    assert!(result.len() > 0);
    assert!(utils::close_rect(
        result[0].0,
        cv::Rect {
            x: 220,
            y: 204,
            width: 168,
            height: 168,
        },
        3,
    ));
}

#[cfg(feature = "gpu")]
fn cascade_model_path() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/cuda_haarcascade_frontalface_default.xml"
    )
}

#[cfg(not(feature = "gpu"))]
fn cascade_model_path() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/haarcascade_frontalface_default.xml"
    )
}
