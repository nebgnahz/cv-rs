extern crate cv;

use cv::*;

#[test]
fn test_get_optimal_dft_size() {
    assert_eq!(get_optimal_dft_size(100), 100);
    assert_eq!(get_optimal_dft_size(1829743), 1843200);
}
