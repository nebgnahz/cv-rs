//! Provide types for matching keypoint descriptors
use core::*;

extern "C" {
    fn cv_descriptor_matcher_match(query_descriptors: *mut CMat,
                                   train_descriptors: *mut CMat,
                                   matches: *mut CVec<DMatch>);
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
#[allow(missing_copy_implementations, missing_debug_implementations)]
pub struct FlannBasedMatcher {}

impl FlannBasedMatcher {
    /// Finds the best match for each descriptor from a query set
    pub fn match_(query_descriptors: &Mat, train_descriptors: &Mat) -> Vec<DMatch> {
        let mut matches = CVec::<DMatch>::default();
        unsafe {
            cv_descriptor_matcher_match(query_descriptors.inner, train_descriptors.inner, &mut matches);
        }
        matches.unpack()
    }
}