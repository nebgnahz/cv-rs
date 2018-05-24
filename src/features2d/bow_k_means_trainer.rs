//! Provide types for matching keypoint descriptors
use *;

enum CBOWKMeansTrainer {}

extern "C" {
    fn cv_bow_trainer_new(cluster_count: i32, term_criteria: *mut CTermCriteria, attempts: i32,  centers: KMeansCenters) -> *mut CBOWKMeansTrainer;
    fn cv_bow_trainer_drop(bow_trainer: *mut CBOWKMeansTrainer);
    fn cv_bow_trainer_add(bow_trainer: *mut CBOWKMeansTrainer, descriptors: *mut CMat);
    fn cv_bow_trainer_cluster(bow_trainer: *mut CBOWKMeansTrainer) -> *mut CMat;
}

/// K-means - based class to train visual vocabulary using the bag of visual words approach
#[derive(Debug)]
pub struct BOWKMeansTrainer {
    value: *mut CBOWKMeansTrainer,
}

/// k-Means centers
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum KMeansCenters {
    /// Select random initial centers in each attempt.
    Random = 0,
    /// Use kmeans++ center initialization by Arthur and Vassilvitskii (Arthur2007).
    Pp = 2,
}

impl Drop for BOWKMeansTrainer {
    fn drop(&mut self) {
        unsafe {
            cv_bow_trainer_drop(self.value);
        }
    }
}

impl BOWKMeansTrainer {
    /// Creates a new maximally stable extremal region extractor criteria.
    pub fn new(        cluster_count:i32, term_criteria: TermCriteria, attempts: i32,  centers: KMeansCenters) -> Self {
        let ptr = unsafe { cv_bow_trainer_new(cluster_count, term_criteria.c_criteria, attempts, centers) };
        Self { value: ptr }
    }

    /// Adds descriptors to a training set
    pub fn add(&mut self, descriptors: &Mat) {
        unsafe {
            cv_bow_trainer_add(self.value, descriptors.inner);
        }
    }

    /// Clusters train descriptors
    pub fn cluster(&mut self) -> Mat {
        let cmat = unsafe { cv_bow_trainer_cluster(self.value) };
        Mat::from_raw(cmat)
    }
}

