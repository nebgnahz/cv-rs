extern crate cv;
mod utils;

use cv::features2d::*;
use cv::*;
use utils::*;

#[test]
fn mser_lenna_detect() {
    let lenna = load_lenna();
    let mser: MSER = MSERBuilder::default().into();
    let (msers, boxes) = mser.detect_regions(&lenna);
    assert_ne!(msers.len(), 0);
    assert_ne!(boxes.len(), 0);
}

#[test]
fn surf_lenna_detect_and_compute() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let mser: SURF = SURFBuilder::default().into();
    let (keypoints, descriptors) = mser.detect_and_compute(&lenna, &mask);
    assert_ne!(keypoints.len(), 0);
    assert_ne!(descriptors.rows, 0);
    assert_ne!(descriptors.cols, 0);
    assert_eq!(keypoints.len() as i32, descriptors.rows);
}

#[test]
fn sift_lenna_detect_and_compute() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let mser: SIFT = SIFTBuilder::default().into();
    let (keypoints, descriptors) = mser.detect_and_compute(&lenna, &mask);
    assert_ne!(keypoints.len(), 0);
    assert_ne!(descriptors.rows, 0);
    assert_ne!(descriptors.cols, 0);
    assert_eq!(keypoints.len() as i32, descriptors.rows);
}

#[test]
fn flann_based_matcher() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let mser: SIFT = SIFTBuilder::default().into();
    let (_, descriptors) = mser.detect_and_compute(&lenna, &mask);

    let descriptor_matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
    let train_descriptors = vec![&descriptors];
    descriptor_matcher.add(&train_descriptors);
    descriptor_matcher.train();
    let result = descriptor_matcher.match_(&descriptors);
    assert_ne!(result.len(), 0);
}

#[test]
fn flann_based_matcher_two() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let mser: SIFT = SIFTBuilder::default().into();
    let (_, descriptors) = mser.detect_and_compute(&lenna, &mask);

    let descriptor_matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
    let result = descriptor_matcher.match_two(&descriptors, &descriptors);
    assert_ne!(result.len(), 0);
}

#[test]
fn flann_based_matcher_knn() {
    const K: usize = 3;
    let lenna = load_lenna();
    let mask = Mat::new();
    let mser: SIFT = SIFTBuilder::default().into();
    let (_, descriptors) = mser.detect_and_compute(&lenna, &mask);

    let descriptor_matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
    let train_descriptors = vec![&descriptors];
    descriptor_matcher.add(&train_descriptors);
    descriptor_matcher.train();
    let result = descriptor_matcher.knn_match(&descriptors, K);
    assert_ne!(result.len(), 0);
    assert_eq!(result.first().unwrap().len(), K);
}
