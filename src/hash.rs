//! The module brings implementations of different image hashing algorithms.
use self::private::*;

use native::cv_average_hash_drop;
use native::cv_average_hash_new;
use native::cv_block_mean_hash_drop;
use native::cv_block_mean_hash_new;
use native::cv_color_moment_hash_drop;
use native::cv_color_moment_hash_new;
use native::cv_img_hash_AverageHash;
use native::cv_img_hash_BlockMeanHash;
use native::cv_img_hash_ColorMomentHash;
use native::cv_img_hash_MarrHildrethHash;
use native::cv_img_hash_PHash;
use native::cv_img_hash_RadialVarianceHash;
use native::cv_marr_hildreth_hash_drop;
use native::cv_marr_hildreth_hash_new;
use native::cv_phash_drop;
use native::cv_phash_new;
use native::cv_radial_variance_hash_drop;
use native::cv_radial_variance_hash_new;

use *;

mod private {
    pub trait HashImpl {
        fn get_value(&self) -> *mut native::cv_Ptr<native::cv_img_hash_ImgHashBase>;
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
            let result = native::cv_mat_new();
            let value = self.get_value();
            native::cv_hash_compute(value, mat.inner, result);
            Mat::from_raw(result)
        }
    }

    /// Compares two image hashes
    fn compare(&self, lhs: &Mat, rhs: &Mat) -> f64 {
        unsafe {
            let value = self.get_value();
            native::cv_hash_compare(value, lhs.inner, rhs.inner)
        }
    }
}

macro_rules! impl_hash {
    ($x:ident, $ctor:ident, $drop:ident, $ty:ident, $description:expr) => {
        #[doc=$description]
        #[derive(Debug)]
        pub struct $x {
            value: *mut native::cv_Ptr<$ty>,
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
            fn get_value(&self) -> *mut native::cv_Ptr<native::cv_img_hash_ImgHashBase> {
                self.value as *mut native::cv_Ptr<native::cv_img_hash_ImgHashBase>
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
    cv_img_hash_AverageHash,
    "Computes average hash value of the input image"
);
impl_hash!(
    BlockMeanHash,
    cv_block_mean_hash_new,
    cv_block_mean_hash_drop,
    cv_img_hash_BlockMeanHash,
    "Image hash based on block mean"
);
impl_hash!(
    ColorMomentHash,
    cv_color_moment_hash_new,
    cv_color_moment_hash_drop,
    cv_img_hash_ColorMomentHash,
    "Image hash based on color moments"
);
impl_hash!(
    MarrHildrethHash,
    cv_marr_hildreth_hash_new,
    cv_marr_hildreth_hash_drop,
    cv_img_hash_MarrHildrethHash,
    "Marr-Hildreth Operator Based Hash, slowest but more discriminative."
);
impl_hash!(
    PHash,
    cv_phash_new,
    cv_phash_drop,
    cv_img_hash_PHash,
    "Slower than AverageHash, but tolerant of minor modifications"
);
impl_hash!(
    RadialVarianceHash,
    cv_radial_variance_hash_new,
    cv_radial_variance_hash_drop,
    cv_img_hash_RadialVarianceHash,
    "Image hash based on Radon transform"
);
