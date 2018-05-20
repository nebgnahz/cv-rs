extern crate cv;
extern crate float_cmp;
mod utils;

use cv::imgcodecs::ImageReadMode;
use cv::imgproc::*;
use cv::*;
use float_cmp::ApproxEqRatio;

const FIRST_IMAGE_PATH: &str = "assets/Histogram_Comparison_Source_0.png";
const SECOND_IMAGE_PATH: &str = "assets/Histogram_Comparison_Source_1.png";

#[test]
#[should_panic]
fn compare_hist_different_dimensions_panic() {
    let first_image = Mat::from_path(FIRST_IMAGE_PATH, ImageReadMode::Color).unwrap();
    let second_image = Mat::from_path(SECOND_IMAGE_PATH, ImageReadMode::Color).unwrap();
    let _ = first_image
        .compare_hist(&second_image, HistogramComparisionMethod::Correlation)
        .unwrap();
}

#[test]
fn compare_hist_correlation() {
    compare_hist(HistogramComparisionMethod::Correlation, 0.204);
}

#[test]
fn compare_hist_chi_square() {
    compare_hist(HistogramComparisionMethod::ChiSquare, 2901.0);
}

#[test]
fn compare_hist_intersection() {
    compare_hist(HistogramComparisionMethod::Intersection, 5.37);
}

#[test]
fn compare_hist_bhattacharyya() {
    compare_hist(HistogramComparisionMethod::Bhattacharyya, 0.679);
}

#[test]
fn compare_hist_chi_square_alternative() {
    compare_hist(HistogramComparisionMethod::ChiSquareAlternative, 39.94);
}

#[test]
fn compare_hist_kullback_leibler_divergence() {
    compare_hist(HistogramComparisionMethod::KullbackLeiblerDivergence, 50.71);
}

fn compare_hist(method: HistogramComparisionMethod, expected_result: f64) {
    let first_image = get_image_histogram(FIRST_IMAGE_PATH);
    let second_image = get_image_histogram(SECOND_IMAGE_PATH);
    let result = first_image.compare_hist(&second_image, method).unwrap();
    assert_eq(result, expected_result);
}

fn get_image_histogram(path: &'static str) -> Mat {
    let image = Mat::from_path(path, ImageReadMode::Color).unwrap();
    let image = image.cvt_color(ColorConversion::BGR2HSV);
    let hsize = [50, 60];
    let h_ranges = [0_f32, 180_f32];
    let s_ranges = [0_f32, 256_f32];
    let ranges = [h_ranges, s_ranges];
    let channels = [0, 1];
    let image = image.calc_hist(&channels, &Mat::new(), &hsize, &ranges);
    let image = image.normalize(0.0, 1.0, NormType::MinMax);
    image
}

fn assert_eq(a: f64, b: f64) {
    assert!(a.approx_eq_ratio(&b, 0.01), format!("{} == {}", a, b));
}
