// This resembles the OpenCV read image example code:
// http://docs.opencv.org/3.1.0/db/deb/tutorial_display_image.html
extern crate rust_vision;
use rust_vision::*;
use rust_vision::cuda::*;
use rust_vision::objdetect::SvmDetector;

fn main() {
    let mat = Mat::from_path("./assets/AVG-TownCentre-test-000011.jpg", 0);
    let mut gpu_mat = GpuMat::default();
    gpu_mat.upload(&mat);

    let detector = SvmDetector::default_people_detector();
    let mut hog = GpuHog::default();
    hog.set_svm_detector(detector);
    let found = hog.detect(&gpu_mat);

    found.iter()
        .map(|&r| mat.rectangle(r.scale(0.6)))
        .count();
    highgui_named_window("Display window", WindowFlags::WindowAutosize);
    mat.show("Display window", 0);
}
