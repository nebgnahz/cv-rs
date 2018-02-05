//! Provide types for matching keypoint descriptors
use core::*;
use std::ffi::*;

enum CDescriptorMatcher {}

extern "C" {
    fn cv_matcher_new(descriptor_matcher_type: &CStr) -> *mut CDescriptorMatcher;
    fn cv_matcher_drop(descriptor_matcher: *mut CDescriptorMatcher);
    fn cv_matcher_add(descriptor_matcher: *mut CDescriptorMatcher, descriptors: *mut CVec<CMat>);
    fn cv_matcher_train(descriptor_matcher: *mut CDescriptorMatcher);
    fn cv_matcher_match(descriptor_matcher: *mut CDescriptorMatcher, query_descriptors: *mut CMat, matches: *mut CVec<DMatch>);
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

/// Flann-based descriptor matcher
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
    pub fn new(descriptor_matcher_type: &str) -> DescriptorMatcher {
        let descriptor_matcher_type = CString::new(descriptor_matcher_type).unwrap();
        let value = unsafe {
            cv_matcher_new(&descriptor_matcher_type)
        };
        DescriptorMatcher { value: value }
    }

    /// Adds descriptors to train a CPU or GPU descriptor collection
    pub fn add(&self, descriptors: Vec<Mat>) {
        unimplemented!()
    }

    /// Trains a descriptor matcher
    pub fn train(&self) {
        unsafe{cv_matcher_train(self.value)}
    }

    /// Finds the best match for each descriptor from a query set
    pub fn match_(&self, query_descriptors: &Mat) -> Vec<DMatch> {
        let mut matches = CVec::<DMatch>::default();
        unsafe {
            cv_matcher_match(self.value,query_descriptors.inner, &mut matches);
        }
        matches.unpack()
    }
}