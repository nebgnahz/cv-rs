//! Various object detection algorithms, such as Haar feature-based cascade
//! classifier for object detection and histogram of oriented gradients (HOG).

use super::core::*;
use super::errors::*;
use super::*;
use failure::Error;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};
use std::path::Path;
use std::vec::Vec;

enum CCascadeClassifier {}

extern "C" {
    fn cv_cascade_classifier_new() -> *mut CCascadeClassifier;
    fn cv_cascade_classifier_load(cc: *mut CCascadeClassifier, p: *const c_char) -> bool;
    fn cv_cascade_classifier_drop(p: *mut CCascadeClassifier);
    fn cv_cascade_classifier_detect(
        cc: *mut CCascadeClassifier,
        cmat: *mut CMat,
        vec_of_rect: *mut CVec<Rect>,
        scale_factor: c_double,
        min_neighbors: c_int,
        flags: c_int,
        min_size: Size2i,
        max_size: Size2i,
    );
}

/// We can safely send the classifier (a mutable pointer) to a different thread
unsafe impl Send for CascadeClassifier {}

/// An object detect trait.
pub trait ObjectDetect {
    /// Detects the object inside this image and returns a list of detections
    /// with their confidence.
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)>;
}

/// Cascade classifier class for object detection.
#[derive(Debug)]
pub struct CascadeClassifier {
    inner: *mut CCascadeClassifier,
}

impl ObjectDetect for CascadeClassifier {
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)> {
        self.detect_multiscale(image)
            .into_iter()
            .map(|r| (r, 0f64))
            .collect::<Vec<_>>()
    }
}

impl CascadeClassifier {
    /// Creates a cascade classifier, uninitialized. Before use, call load.
    pub fn new() -> CascadeClassifier {
        CascadeClassifier {
            inner: unsafe { cv_cascade_classifier_new() },
        }
    }

    /// Creates a cascade classifier using the model specified.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let cc = CascadeClassifier::new();
        cc.load(path)?;
        Ok(cc)
    }

    /// Loads the classifier model from a path.
    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        if let Some(p) = path.as_ref().to_str() {
            let s = CString::new(p)?;
            if unsafe { cv_cascade_classifier_load(self.inner, (&s).as_ptr()) } {
                return Ok(());
            }
        }

        Err(CvError::InvalidPath(path.as_ref().to_path_buf()).into())
    }

    /// The default detection uses scale factor 1.1, minNeighbors 3, no min size
    /// or max size.
    pub fn detect_multiscale(&self, mat: &Mat) -> Vec<Rect> {
        self.detect_with_params(mat, 1.1, 3, Size2i::default(), Size2i::default())
    }

    /// Detects the object using parameters specified.
    ///
    /// * `mat` - Matrix of the type CV_8U containing an image where objects are
    ///   detected.
    /// * `scale_factor` - Parameter specifying how much the image size is
    ///   reduced at each image scale.
    /// * `min_neighbors` - Parameter specifying how many neighbors each
    ///   candidate rectangle should have to retain it.
    /// * `min_size` - Minimum possible object size. Objects smaller than that
    ///   are ignored.
    /// * `max_size` - Maximum possible object size. Objects larger than that
    ///   are ignored
    ///
    /// OpenCV has a parameter (`flags`) that's not used at all.
    pub fn detect_with_params(
        &self,
        mat: &Mat,
        scale_factor: f32,
        min_neighbors: c_int,
        min_size: Size2i,
        max_size: Size2i,
    ) -> Vec<Rect> {
        let mut c_result = CVec::<Rect>::default();
        unsafe {
            cv_cascade_classifier_detect(
                self.inner,
                mat.inner,
                &mut c_result,
                scale_factor as c_double,
                min_neighbors,
                0,
                min_size,
                max_size,
            )
        }
        c_result.unpack()
    }
}

impl Drop for CascadeClassifier {
    fn drop(&mut self) {
        unsafe {
            cv_cascade_classifier_drop(self.inner);
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Opaque type for C/C++ SvmDetector object
pub enum CSvmDetector {}

/// SvmDetector
#[derive(Debug)]
pub struct SvmDetector {
    /// Pointer to the inner data structure
    pub(crate) inner: *mut CSvmDetector,
}

extern "C" {
    fn cv_hog_default_people_detector() -> *mut CSvmDetector;
    fn cv_hog_daimler_people_detector() -> *mut CSvmDetector;
    fn cv_hog_detector_drop(d: *mut CSvmDetector);
}

impl SvmDetector {
    /// The built-in people detector.
    ///
    /// The size of the default people detector is 64x128, that mean that the
    /// people you would want to detect have to be atleast 64x128.
    pub fn default_people_detector() -> SvmDetector {
        SvmDetector {
            inner: unsafe { cv_hog_default_people_detector() },
        }
    }

    /// Returns the Daimler people detector.
    pub fn daimler_people_detector() -> SvmDetector {
        SvmDetector {
            inner: unsafe { cv_hog_daimler_people_detector() },
        }
    }
}

impl Drop for SvmDetector {
    fn drop(&mut self) {
        unsafe {
            cv_hog_detector_drop(self.inner);
        }
    }
}

/// Parameters that controls the behavior of HOG.
#[derive(Debug, Clone, Copy)]
pub struct HogParams {
    /// Detection window size. Align to block size and block stride. The default
    /// is 64x128, trained the same as original paper.
    pub win_size: Size2i,

    /// Block size in pixels. Align to cell size. Only (16,16) is supported for
    /// now (at least for GPU).
    pub block_size: Size2i,

    /// Block stride. It must be a multiple of cell size.
    pub block_stride: Size2i,

    /// Cell size. Only (8, 8) is supported for now.
    pub cell_size: Size2i,

    /// Number of bins. Only 9 bins per cell are supported for now.
    pub nbins: c_int,

    /// Gaussian smoothing window parameter. Default -1 for CPU and 4.0 for GPU.
    pub win_sigma: f64,

    /// L2-Hys normalization method shrinkage. Default 0.2.
    pub l2hys_threshold: f64,

    /// Flag to specify whether the gamma correction preprocessing is required
    /// or not. Default false.
    pub gamma_correction: bool,

    /// Maximum number of detection window increases (HOG scales). Default: 64.
    pub nlevels: usize,

    // =======================================================================
    //  Functions from detect function
    // =======================================================================
    /// Threshold for the distance between features and SVM classifying
    /// plane. Usually it is 0 and should be specfied in the detector
    /// coefficients (as the last free coefficient). But if the free coefficient
    /// is omitted (which is allowed), you can specify it manually here.
    pub hit_threshold: f64,

    /// Window stride. It must be a multiple of block stride.
    pub win_stride: Size2i,

    /// Padding
    pub padding: Size2i,

    /// Coefficient of the detection window increase.
    pub scale: f64,

    /// Coefficient to regulate the similarity threshold. When detected, some
    /// objects can be covered by many rectangles. 0 means not to perform
    /// grouping.
    pub group_threshold: c_int,

    /// The useMeanShiftGrouping parameter is a boolean indicating whether or
    /// not mean-shift grouping should be performed to handle potential
    /// overlapping bounding boxes. While this value should not be set and users
    /// should employ non-maxima suppression instead, we support setting it as a
    /// library function.
    pub use_meanshift_grouping: bool,

    /// The `finalThreshold` parameter is mainly used to select the clusters
    /// that have at least `finalThreshold + 1` rectangles. This parameter is
    /// passed when meanShift is enabled; the function rejects the small
    /// clusters containing less than or equal to `finalThreshold` rectangles,
    /// computes the average rectangle size for the rest of the accepted
    /// clusters and adds those to the output rectangle list.
    pub final_threshold: f64,
}

const DEFAULT_WIN_SIGMA: f64 = -1f64;
const DEFAULT_NLEVELS: usize = 64;

impl Default for HogParams {
    fn default() -> HogParams {
        let win_sigma = {
            if cfg!(feature = "cuda") {
                4.0
            } else {
                DEFAULT_WIN_SIGMA
            }
        };

        HogParams {
            win_size: Size2i::new(64, 128),
            block_size: Size2i::new(16, 16),
            block_stride: Size2i::new(8, 8),
            cell_size: Size2i::new(8, 8),
            nbins: 9,

            win_sigma: win_sigma,
            l2hys_threshold: 0.2,
            gamma_correction: false,
            nlevels: DEFAULT_NLEVELS,

            hit_threshold: 0f64,
            win_stride: Size2i::new(8, 8),
            padding: Size2i::default(),
            scale: 1.05,
            group_threshold: 2,

            final_threshold: 2.0,
            use_meanshift_grouping: false,
        }
    }
}

enum CHogDescriptor {}

/// `HogDescriptor` implements Histogram of Oriented Gradients.
#[derive(Debug)]
pub struct HogDescriptor {
    inner: *mut CHogDescriptor,

    /// Hog parameters.
    pub params: HogParams,
}

unsafe impl Send for HogDescriptor {}

extern "C" {
    fn cv_hog_new() -> *mut CHogDescriptor;
    fn cv_hog_drop(hog: *mut CHogDescriptor);
    fn cv_hog_set_svm_detector(hog: *mut CHogDescriptor, svm: *mut CSvmDetector);
    fn cv_hog_detect(
        hog: *mut CHogDescriptor,
        image: *mut CMat,
        objs: *mut CVec<Rect>,
        weights: *mut CVec<c_double>,
        win_stride: Size2i,
        padding: Size2i,
        scale: c_double,
        final_threshold: c_double,
        use_means_shift: bool,
    );
}

impl Default for HogDescriptor {
    fn default() -> HogDescriptor {
        HogDescriptor {
            inner: unsafe { cv_hog_new() },
            params: HogParams::default(),
        }
    }
}

impl ObjectDetect for HogDescriptor {
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)> {
        let mut detected = CVec::<Rect>::default();
        let mut weights = CVec::<c_double>::default();
        unsafe {
            cv_hog_detect(
                self.inner,
                image.inner,
                &mut detected,
                &mut weights,
                self.params.win_stride,
                self.params.padding,
                self.params.scale,
                self.params.final_threshold,
                self.params.use_meanshift_grouping,
            )
        }

        let results = detected.unpack();
        let weights = weights.unpack();
        results.into_iter().zip(weights).collect::<Vec<_>>()
    }
}

impl HogDescriptor {
    /// Creates a HogDescriptor with provided parameters.
    pub fn with_params(params: HogParams) -> HogDescriptor {
        HogDescriptor {
            inner: unsafe { cv_hog_new() },
            params: params,
        }
    }

    /// Sets the SVM detector.
    pub fn set_svm_detector(&mut self, detector: SvmDetector) {
        unsafe { cv_hog_set_svm_detector(self.inner, detector.inner) }
    }
}

impl Drop for HogDescriptor {
    fn drop(&mut self) {
        unsafe { cv_hog_drop(self.inner) }
    }
}
