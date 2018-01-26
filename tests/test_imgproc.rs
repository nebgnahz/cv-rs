extern crate cv;
extern crate float_cmp;
mod utils;

use cv::*;
use cv::imgproc::*;
use float_cmp::ApproxEqRatio;
use utils::*;

#[test]
#[should_panic]
fn compare_hist_different_dimensions_panic() {
    let first_image = load_unchanged("assets/Histogram_Comparison_Source_0.jpg");
    let second_image = load_unchanged("assets/Histogram_Comparison_Source_1.jpg");
    let _ = first_image.compare_hist(&second_image, HistogramComparisionMethod::Corellation).unwrap();
}

#[test]
fn compare_hist_correlation() {
    compare_hist(HistogramComparisionMethod::Corellation, 0.211);
}

#[test]
fn compare_hist_chi_square() {
    compare_hist(HistogramComparisionMethod::ChiSquare, 1360.7);
}

#[test]
fn compare_hist_intersection() {
    compare_hist(HistogramComparisionMethod::Intersection, 5.682);
}

#[test]
fn compare_hist_bhattacharyya() {
    compare_hist(HistogramComparisionMethod::Bhattacharyya, 0.6679);
}

#[test]
fn compare_hist_chi_square_alternative() {
    compare_hist(HistogramComparisionMethod::ChiSquareAlternative, 41.027);
}

#[test]
fn compare_hist_kullback_leibler_divergence() {
    compare_hist(
        HistogramComparisionMethod::KullbackLeiblerDivergence,
        54.06287,
    );
}

fn compare_hist(method: HistogramComparisionMethod, expected_result: f64) {
    let first_image = get_image_histogram("assets/Histogram_Comparison_Source_0.jpg");
    let second_image = get_image_histogram("assets/Histogram_Comparison_Source_1.jpg");
    let result = first_image.compare_hist(&second_image, method).unwrap();
    assert_eq(result, expected_result);
}

fn get_image_histogram(path: &'static str) -> Mat {
    let image = load_unchanged(path);
    let image = image.cvt_color(ColorConversionCodes::BGR2HSV);
    let hsize = [50, 60];
    let h_ranges = [0_f32, 180_f32];
    let s_ranges = [0_f32, 256_f32];
    let ranges = [
        h_ranges.as_ptr() as *const f32,
        s_ranges.as_ptr() as *const f32,
    ];
    let channels = [0, 1];
    let image = image.calc_hist(
        channels.as_ptr(),
        Mat::new(),
        2,
        hsize.as_ptr(),
        ranges.as_ptr(),
    );
    let image = image.normalize(0_f64, 1_f64, NormTypes::NormMinMax);
    image
}

fn assert_eq(a: f64, b: f64) {
    assert!(a.approx_eq_ratio(&b, 0.001), format!("{} == {}", a, b));
}
