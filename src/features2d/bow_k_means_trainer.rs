//! Provide types for matching keypoint descriptors
use *;

/// K-means - based class to train visual vocabulary using the bag of visual words approach
#[derive(Debug)]
pub struct BOWKMeansTrainer {
    value: *mut native::cvsys_BOWKMeansTrainer,
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
            native::cvsys_bow_trainer_drop(self.value);
        }
    }
}

impl BOWKMeansTrainer {
    /// Creates a new maximally stable extremal region extractor criteria.
    pub fn new(cluster_count: i32, term_criteria: TermCriteria, attempts: i32, centers: KMeansCenters) -> Self {
        let ptr = unsafe { native::cvsys_bow_trainer_new(cluster_count, term_criteria.c_criteria, attempts, centers as i32) };
        Self { value: ptr }
    }

    /// Adds descriptors to a training set
    pub fn add(&mut self, descriptors: &Mat) {
        unsafe {
            native::cvsys_bow_trainer_add(self.value, descriptors.inner);
        }
    }

    /// Clusters train descriptors
    pub fn cluster(&mut self) -> Mat {
        unsafe {
            let cmat = native::cvsys_bow_trainer_cluster(self.value);
            Mat::from_raw(cmat)
        }
    }
}
