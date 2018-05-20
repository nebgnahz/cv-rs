//! Provide types for matching keypoint descriptors
use std::os::raw::{c_char, c_int};
use *;

enum CDescriptorMatcher {}

extern "C" {
    fn cv_matcher_new(descriptor_matcher_type: *const c_char) -> *mut CDescriptorMatcher;
    fn cv_matcher_drop(descriptor_matcher: *mut CDescriptorMatcher);
    fn cv_matcher_add(descriptor_matcher: *mut CDescriptorMatcher, descriptors: *const CVecView<*mut CMat>);
    fn cv_matcher_train(descriptor_matcher: *mut CDescriptorMatcher);
    fn cv_matcher_is_empty(descriptor_matcher: *mut CDescriptorMatcher) -> bool;
    fn cv_matcher_match(
        descriptor_matcher: *mut CDescriptorMatcher,
        query_descriptors: *mut CMat,
        matches: *mut CVec<DMatch>,
    );
    fn cv_matcher_match_two(
        descriptor_matcher: *mut CDescriptorMatcher,
        query_descriptors: *mut CMat,
        train_descriptors: *mut CMat,
        matches: *mut CVec<DMatch>,
    );
    fn cv_matcher_knn_match(
        descriptor_matcher: *mut CDescriptorMatcher,
        query_descriptors: *mut CMat,
        k: c_int,
        matches: *mut CVec<CVec<DMatch>>,
    );
}

/// Type for matching keypoint descriptors
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct DMatch {
    distance: f32,
    img_idx: i32,
    query_idx: i32,
    train_idx: i32,
}

/// Descriptor matcher type
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum DescriptorMatcherType {
    BruteForce,
    BruteForceL1,
    BruteForceHamming,
    BruteForceHamming2,
    FlannBased,
}

impl DescriptorMatcherType {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            DescriptorMatcherType::BruteForce => "BruteForce",
            DescriptorMatcherType::BruteForceL1 => "BruteForce-L1",
            DescriptorMatcherType::BruteForceHamming => "BruteForce-Hamming",
            DescriptorMatcherType::BruteForceHamming2 => "BruteForce-Hamming(2)",
            DescriptorMatcherType::FlannBased => "FlannBased",
        }
    }
}

/// Type for matching keypoint descriptors
#[derive(Debug)]
pub struct DescriptorMatcher {
    value: *mut CDescriptorMatcher,
}

impl Drop for DescriptorMatcher {
    fn drop(&mut self) {
        unsafe {
            cv_matcher_drop(self.value);
        }
    }
}

impl DescriptorMatcher {
    /// Creates a descriptor matcher of a given type with the default parameters (using default constructor).
    pub fn new(descriptor_matcher_type: DescriptorMatcherType) -> DescriptorMatcher {
        let descriptor_matcher_type = CString::new(descriptor_matcher_type.as_str()).unwrap();
        let value = unsafe { cv_matcher_new(descriptor_matcher_type.as_ptr()) };
        DescriptorMatcher { value: value }
    }

    /// Adds descriptors to train a CPU or GPU descriptor collection
    pub fn add(&self, descriptors: &Vec<&Mat>) {
        let descriptors = descriptors.iter().map(|x| x.inner).collect();
        let vec_view = CVecView::pack(&descriptors);
        unsafe {
            cv_matcher_add(self.value, &vec_view);
        }
    }

    /// Trains a descriptor matcher
    pub fn train(&self) {
        unsafe { cv_matcher_train(self.value) }
    }

    /// Returns true if there are no train descriptors
    pub fn is_empty(&self) -> bool {
        unsafe { cv_matcher_is_empty(self.value) }
    }

    /// Finds the best match for each descriptor from a query set
    pub fn match_(&self, query_descriptors: &Mat) -> Vec<DMatch> {
        let mut matches = CVec::<DMatch>::default();
        unsafe {
            cv_matcher_match(self.value, query_descriptors.inner, &mut matches);
        }
        matches.unpack()
    }

    /// Finds the best match for each descriptor from a query set.
    /// Unlike `match_`, train descriptors collection are passed directly
    pub fn match_two(&self, query_descriptors: &Mat, train_descriptors: &Mat) -> Vec<DMatch> {
        let mut matches = CVec::<DMatch>::default();
        unsafe {
            cv_matcher_match_two(
                self.value,
                query_descriptors.inner,
                train_descriptors.inner,
                &mut matches,
            );
        }
        matches.unpack()
    }

    /// Finds the k best matches for each descriptor from a query set.
    pub fn knn_match(&self, query_descriptors: &Mat, k: usize) -> Vec<Vec<DMatch>> {
        let mut matches = CVec::<CVec<DMatch>>::default();
        unsafe {
            cv_matcher_knn_match(self.value, query_descriptors.inner, k as c_int, &mut matches);
        }
        matches.unpack()
    }
}
