extern crate cv;
mod utils;

use cv::*;

#[test]
fn test_split() {
    let mat = Mat::from_buffer(3, 1, cv::core::CvType::Cv8UC2, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(mat.channels, 2);
    let split = mat.split();
    assert_eq!(split[0].data(), &[1, 3, 5]);
    assert_eq!(split[0].channels, 1);
    assert_eq!(split[0].cv_type(),  cv::core::CvType::Cv8UC1);
    assert_eq!(split[1].data(), &[2, 4, 6]);
    assert_eq!(split[1].channels, 1);
    assert_eq!(split[1].cv_type(),  cv::core::CvType::Cv8UC1);
}
