//! Provides different algorithms for text detection and recognition in natural scene images
#[macro_use]
mod phash;
pub use self::phash::*;

use mat::CMat;
use *;

extern "C" {
    fn cv_hash_compute(phash: *const private::CHash, mat: *const CMat, result: *mut CMat);
    fn cv_hash_compare(phash: *const private::CHash, lhs: *const CMat, rhs: *mut CMat) -> f64;
}

mod private {
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub enum CHash {}

    pub trait HashImpl {
        fn get_value(&self) -> *mut CHash;
    }
}

#[allow(missing_docs)]
pub trait HashImplInterface: private::HashImpl {}

/// Basic trait for all OCR types
pub trait Hash {
    /// Computes image hash
    fn compute(&self, mat: &Mat) -> Mat;

    /// Compares two image hashes
    fn compare(&self, lhs: &Mat, rhs: &Mat) -> f64;
}

impl<T: HashImplInterface> Hash for T {
    /// Computes image hash
    fn compute(&self, mat: &Mat) -> Mat {
        let result = CMat::new();
        let value = self.get_value();
        unsafe { cv_hash_compute(value, mat.inner, result) };
        Mat::from_raw(result)
    }

    /// Compares two image hashes
    fn compare(&self, lhs: &Mat, rhs: &Mat) -> f64 {
        let value = self.get_value();
        unsafe { cv_hash_compare(value, lhs.inner, rhs.inner) }
    }
}
