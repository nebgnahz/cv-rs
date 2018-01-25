extern crate cv;
mod utils;

use cv::imgproc::*;
use utils::*;

#[test]
fn compare_hist() {
    let first_image = load_unchanged("assets/Histogram_Comparison_Source_0.jpg");
    let second_image = load_unchanged("assets/Histogram_Comparison_Source_1.jpg");
    let result = first_image.compare_hist(&second_image, HistogramComparisionMethod::ChiSquare).unwrap();
    assert_eq!(0f64, result);
}
