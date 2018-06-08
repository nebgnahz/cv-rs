//! Provides different algorithms for text detection and recognition in natural scene images
use self::private::*;

use mat::CMat;
use *;

extern "C" {
    fn cv_hash_compute(phash: *const CHash, mat: *const CMat, result: *mut CMat);
    fn cv_hash_compare(phash: *const CHash, lhs: *const CMat, rhs: *mut CMat) -> f64;

    fn cv_average_hash_new() -> *mut CHash;
    fn cv_average_hash_drop(phash: *mut CHash);
    fn cv_block_mean_hash_new() -> *mut CHash;
    fn cv_block_mean_hash_drop(phash: *mut CHash);
    fn cv_color_moment_hash_new() -> *mut CHash;
    fn cv_color_moment_hash_drop(phash: *mut CHash);
    fn cv_marr_hildreth_hash_new() -> *mut CHash;
    fn cv_marr_hildreth_hash_drop(phash: *mut CHash);
    fn cv_phash_new() -> *mut CHash;
    fn cv_phash_drop(phash: *mut CHash);
    fn cv_radial_variance_hash_new() -> *mut CHash;
    fn cv_radial_variance_hash_drop(phash: *mut CHash);
}

mod private {
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub enum CHash {}

    pub trait HashImpl {
        fn get_value(&self) -> *mut CHash;
    }
}

#[allow(missing_docs)]
pub trait HashImplInterface: HashImpl {}

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

/// AverageHash
#[derive(Debug)]
pub struct AverageHash {
    value: *mut CHash,
}

/// BlockMeanHash
#[derive(Debug)]
pub struct BlockMeanHash {
    value: *mut CHash,
}

/// ColorMomentHash
#[derive(Debug)]
pub struct ColorMomentHash {
    value: *mut CHash,
}

/// MarrHildrethHash
#[derive(Debug)]
pub struct MarrHildrethHash {
    value: *mut CHash,
}

/// Slower than average_hash, but tolerant of minor modifications
#[derive(Debug)]
pub struct PHash {
    value: *mut CHash,
}

/// RadialVarianceHash
#[derive(Debug)]
pub struct RadialVarianceHash {
    value: *mut CHash,
}

macro_rules! impl_hash {
    ($x:ident, $ctor:ident, $drop:ident) => {
        impl $x {
            /// Creates new $x
            pub fn new() -> Self {
                let value = unsafe { $ctor() };
                Self { value }
            }
        }

        impl Drop for $x {
            fn drop(&mut self) {
                unsafe {
                    $drop(self.value);
                }
            }
        }

        impl HashImpl for $x {
            fn get_value(&self) -> *mut CHash {
                self.value
            }
        }

        impl HashImplInterface for $x {}
    };
}

impl_hash!(AverageHash, cv_average_hash_new, cv_average_hash_drop);
impl_hash!(BlockMeanHash, cv_block_mean_hash_new, cv_block_mean_hash_drop);
impl_hash!(ColorMomentHash, cv_color_moment_hash_new, cv_color_moment_hash_drop);
impl_hash!(MarrHildrethHash, cv_marr_hildreth_hash_new, cv_marr_hildreth_hash_drop);
impl_hash!(PHash, cv_phash_new, cv_phash_drop);
impl_hash!(
    RadialVarianceHash,
    cv_radial_variance_hash_new,
    cv_radial_variance_hash_drop
);
