//! Provide 2D image feature detectors and descriptor extractors
mod mser;
mod surf;
mod sift;
mod descriptor_matchers;

use core::*;

pub use self::mser::*;
pub use self::sift::*;
pub use self::surf::*;
pub use self::descriptor_matchers::*;

/// Basic trait for 2D image feature detectors and descriptor extractors
pub trait Feature2D {
    /// Detects keypoints and computes the descriptors
    fn detect_and_compute(&self, image: &Mat, mask: &Mat) -> (Vec<KeyPoint>, Mat);
}
