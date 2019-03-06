//! The module brings implementations of different image hashing algorithms.
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
        fn get_value(&self) -> *const CHash;
    }
}

#[allow(missing_docs)]
pub trait HashImplInterface: HashImpl {}

/// Basic trait for all hash types
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

macro_rules! impl_hash {
    ($x:ident, $ctor:ident, $drop:ident, $description:expr) => {
        #[doc=$description]
        #[derive(Debug)]
        pub struct $x {
            value: *const CHash,
        }

        impl $x {
            /// Creates new instance
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl Drop for $x {
            fn drop(&mut self) {
                unsafe {
                    $drop(self.value as *mut _);
                }
            }
        }

        impl Default for $x {
            fn default() -> Self {
                let value = unsafe { $ctor() };
                Self { value }
            }
        }

        impl HashImpl for $x {
            fn get_value(&self) -> *const CHash {
                self.value
            }
        }

        impl HashImplInterface for $x {}

        // We know that this pointer is used for calling virtual pure functions,
        // But Rust doesn't allow us to share unsafe pointers between threads.
        // However, it's safe because the only place we mutate the pointer is `drop`,
        // Which makes the value inaccessible, so we're ok here too
        unsafe impl Send for $x {}
        unsafe impl Sync for $x {}
    };
}

impl_hash!(
    AverageHash,
    cv_average_hash_new,
    cv_average_hash_drop,
    "Computes average hash value of the input image"
);
impl_hash!(
    BlockMeanHash,
    cv_block_mean_hash_new,
    cv_block_mean_hash_drop,
    "Image hash based on block mean"
);
impl_hash!(
    ColorMomentHash,
    cv_color_moment_hash_new,
    cv_color_moment_hash_drop,
    "Image hash based on color moments"
);
impl_hash!(
    MarrHildrethHash,
    cv_marr_hildreth_hash_new,
    cv_marr_hildreth_hash_drop,
    "Marr-Hildreth Operator Based Hash, slowest but more discriminative."
);
impl_hash!(
    PHash,
    cv_phash_new,
    cv_phash_drop,
    "Slower than AverageHash, but tolerant of minor modifications"
);
impl_hash!(
    RadialVarianceHash,
    cv_radial_variance_hash_new,
    cv_radial_variance_hash_drop,
    "Image hash based on Radon transform"
);
