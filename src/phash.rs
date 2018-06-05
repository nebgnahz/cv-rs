//! PHash

use *;

#[derive(Clone, Copy, Debug)]
enum CPHash {}

impl CPHash {

}

extern "C" {
    fn cv_phash_new() -> *mut CPHash;
    fn cv_phash_drop(phash: *mut CPHash);
    fn cv_phash_compute(phash: *mut CPHash, mat: *const CMat, result: *mut CMat);
}

/// Slower than average_hash, but tolerant of minor modifications
#[derive(Debug)]
pub struct PHash {
    value: *mut CPHash,
}

impl PHash {
    /// Creates new PHash
    pub fn new() -> PHash {
        let value = unsafe { cv_phash_new() };
        Self {
            value
        }
    }

    /// Computes image hash
    pub fn compute(&self, mat: &Mat) -> Mat {
        let result = CMat::new();
        unsafe { cv_phash_compute(self.value, mat.inner, result) };
        Mat::from_raw(result)
    }
}

impl Drop for PHash {
    fn drop(&mut self) {
        unsafe {
            cv_phash_drop(self.value);
        }
    }
}