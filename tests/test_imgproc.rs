extern crate cv;
extern crate float_cmp;
mod utils;

use cv::*;
use cv::imgproc::*;
use utils::*;
use float_cmp::ApproxEqRatio;

#[test]
fn compare_hist() {
    let first_image = get_image_histogram("assets/Histogram_Comparison_Source_0.jpg");
    let second_image = get_image_histogram("assets/Histogram_Comparison_Source_1.jpg");
    let result = first_image.compare_hist(&second_image, HistogramComparisionMethod::Corellation).unwrap();
    assert_eq(result, 0.211);
}

fn get_image_histogram(path: &'static str) -> Mat {
    let image = load_unchanged(path);
    let image = image.cvt_color(ColorConversionCodes::BGR2HSV);
    let hsize = [50, 60];
    let h_ranges = [0_f32, 180_f32];
    let s_ranges = [0_f32, 256_f32];
    let ranges = [h_ranges.as_ptr() as *const f32, s_ranges.as_ptr() as *const f32];
    let channels = [0, 1];
    let image = image.calc_hist(channels.as_ptr(), Mat::new(), 2, hsize.as_ptr(), ranges.as_ptr());
    let image = image.normalize(0_f64, 1_f64, NormTypes::NormMinMax);
    image
}

fn assert_eq(a: f64, b: f64) {
    assert!(a.approx_eq_ratio(&b,0.0001), format!("{} == {}", a, b));
}