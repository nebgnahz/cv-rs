extern crate cv;
mod utils;

use cv::features2d::*;
use cv::*;
use utils::*;

#[test]
fn mser_lenna_detect() {
    let lenna = load_lenna();
    let mser: Mser = MserBuilder::default().into();
    let (msers, boxes) = mser.detect_regions(&lenna);
    assert_ne!(msers.len(), 0);
    assert_ne!(boxes.len(), 0);
}

#[test]
fn surf_lenna_detect_and_compute() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let surf: SURF = SURFBuilder::default().into();
    let (keypoints, descriptors) = surf.detect_and_compute(&lenna, &mask);
    assert_ne!(keypoints.len(), 0);
    assert_ne!(descriptors.rows, 0);
    assert_ne!(descriptors.cols, 0);
    assert_eq!(keypoints.len() as i32, descriptors.rows);
}

#[test]
fn sift_lenna_detect_and_compute() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (keypoints, descriptors) = sift.detect_and_compute(&lenna, &mask);
    assert_ne!(keypoints.len(), 0);
    assert_ne!(descriptors.rows, 0);
    assert_ne!(descriptors.cols, 0);
    assert_eq!(keypoints.len() as i32, descriptors.rows);
}

#[test]
fn flann_based_matcher() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (_, descriptors) = sift.detect_and_compute(&lenna, &mask);

    let mut descriptor_matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
    descriptor_matcher.add(Some(&descriptors));
    descriptor_matcher.train();
    let result = descriptor_matcher.match_(&descriptors);
    assert_ne!(result.len(), 0);
}

#[test]
fn flann_based_matcher_two() {
    let lenna = load_lenna();
    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (_, descriptors) = sift.detect_and_compute(&lenna, &mask);

    let descriptor_matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
    let result = descriptor_matcher.match_two(&descriptors, &descriptors);
    assert_ne!(result.len(), 0);
}

#[test]
fn flann_based_matcher_knn() {
    const K: usize = 3;
    let lenna = load_lenna();
    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (_, descriptors) = sift.detect_and_compute(&lenna, &mask);

    let mut descriptor_matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
    descriptor_matcher.add(Some(&descriptors));
    descriptor_matcher.train();
    let result = descriptor_matcher.knn_match(&descriptors, K);
    assert_ne!(result.len(), 0);
    assert_eq!(result.first().unwrap().len(), K);
}

#[test]
fn bow() {
    let mut bow = BOWKMeansTrainer::new(2, TermCriteria::new(TermType::Count, 100, 0.01), 1, KMeansCenters::Pp);
    let lenna = load_lenna();
    let messi = load_messi_color();
    let sift: SIFT = SIFTBuilder::default().into();
    let mask = Mat::new();

    let (_, lenna_descriptors) = sift.detect_and_compute(&lenna, &mask);
    let (_, messi_descriptors) = sift.detect_and_compute(&messi, &mask);

    bow.add(&lenna_descriptors);
    bow.add(&messi_descriptors);

    let mat = bow.cluster();

    assert_ne!(mat.cols, 0);
}
