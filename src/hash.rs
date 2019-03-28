//! The module brings implementations of different image hashing algorithms.
use self::private::*;

use native::cvsys_AverageHash;
use native::cvsys_BlockMeanHash;
use native::cvsys_ColorMomentHash;
use native::cvsys_MarrHildrethHash;
use native::cvsys_PHash;
use native::cvsys_RadialVarianceHash;
use native::cvsys_average_hash_drop;
use native::cvsys_average_hash_new;
use native::cvsys_block_mean_hash_drop;
use native::cvsys_block_mean_hash_new;
use native::cvsys_color_moment_hash_drop;
use native::cvsys_color_moment_hash_new;
use native::cvsys_marr_hildreth_hash_drop;
use native::cvsys_marr_hildreth_hash_new;
use native::cvsys_phash_drop;
use native::cvsys_phash_new;
use native::cvsys_radial_variance_hash_drop;
use native::cvsys_radial_variance_hash_new;

use *;

mod private {
    pub trait HashImpl {
        fn get_value(&self) -> *mut super::cvsys_PHash;
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
        unsafe {
            let result = native::cvsys_mat_new();
            native::cvsys_hash_any_compute(self.get_value(), mat.inner, result);
            Mat::from_raw(result)
        }
    }

    /// Compares two image hashes
    fn compare(&self, lhs: &Mat, rhs: &Mat) -> f64 {
        unsafe { native::cvsys_hash_any_compare(self.get_value(), lhs.inner, rhs.inner) }
    }
}

macro_rules! impl_hash {
    ($x:ident, $nat:ident, $ctor:ident, $drop:ident, $description:expr) => {
        #[doc=$description]
        #[derive(Debug)]
        pub struct $x {
            value: *mut $nat,
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
                    $drop(self.value);
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
            fn get_value(&self) -> *mut cvsys_PHash {
                self.value as *mut _
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
    cvsys_AverageHash,
    cvsys_average_hash_new,
    cvsys_average_hash_drop,
    "Computes average hash value of the input image"
);
impl_hash!(
    BlockMeanHash,
    cvsys_BlockMeanHash,
    cvsys_block_mean_hash_new,
    cvsys_block_mean_hash_drop,
    "Image hash based on block mean"
);
impl_hash!(
    ColorMomentHash,
    cvsys_ColorMomentHash,
    cvsys_color_moment_hash_new,
    cvsys_color_moment_hash_drop,
    "Image hash based on color moments"
);
impl_hash!(
    MarrHildrethHash,
    cvsys_MarrHildrethHash,
    cvsys_marr_hildreth_hash_new,
    cvsys_marr_hildreth_hash_drop,
    "Marr-Hildreth Operator Based Hash, slowest but more discriminative."
);
impl_hash!(
    PHash,
    cvsys_PHash,
    cvsys_phash_new,
    cvsys_phash_drop,
    "Slower than AverageHash, but tolerant of minor modifications"
);
impl_hash!(
    RadialVarianceHash,
    cvsys_RadialVarianceHash,
    cvsys_radial_variance_hash_new,
    cvsys_radial_variance_hash_drop,
    "Image hash based on Radon transform"
);
