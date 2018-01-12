extern crate cv;

#[cfg(feature = "gpu")]
use cv::cuda::GpuHog as Hog;
#[cfg(not(feature = "gpu"))]
use cv::objdetect::HogDescriptor as Hog;

#[cfg(feature = "gpu")]
use cv::cuda::GpuCascade as CascadeClassifier;

use cv::objdetect::SvmDetector;
use cv::objdetect::HogParams;
use cv::objdetect::ObjectDetect;
mod utils;

/// This test will run regardless of cuda or not. When tested with `--features
/// cuda`, it will use CUDA-enabled HOG.
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
#[cfg(feature = "gpu")]
fn test_cascade_lenna() {
    let mat = utils::load_lenna();
    let mut d = ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/haarcascade_frontalface_default.xml");
    let cascade = CascadeClassifier::from_path(d).unwrap();
    let result = cascade.detect(&mat);
    assert!(result.len() > 1);
}
