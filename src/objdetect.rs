//! Various object detection algorithms, such as Haar feature-based cascade
//! classifier for object detection and histogram of oriented gradients (HOG).

use libc::{c_double, c_char, c_int};
use std::ffi::CString;
use std::path::Path;
use std::vec::Vec;
use super::core::*;

/// An object detect trait.
pub trait ObjectDetect {
    /// Detects the object inside this image and returns a list of detections
    /// with their confidence.
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)>;
}

/// The opaque type for C
enum CCascadeClassifier {}

/// Cascade classifier class for object detection.
#[derive(Debug)]
pub struct CascadeClassifier {
    inner: *mut CCascadeClassifier,
}

/// We can safely send the classifier (a mutable pointer) to a different thread
unsafe impl Send for CascadeClassifier {}

extern "C" {
    fn opencv_cascade_classifier_new() -> *mut CCascadeClassifier;
    fn opencv_cascade_classifier_load(cc: *mut CCascadeClassifier, p: *const c_char) -> bool;
    fn opencv_cascade_classifier_drop(p: *mut CCascadeClassifier);
    fn opencv_cascade_classifier_detect(cc: *mut CCascadeClassifier,
                                        cmat: *mut CMat,
                                        vec_of_rect: *mut CVecOfRect,
                                        scale_factor: c_double,
                                        min_neighbors: c_int,
                                        flags: c_int,
                                        min_size: Size2i,
                                        max_size: Size2i);
}

impl ObjectDetect for CascadeClassifier {
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)> {
        self.detect_multiscale(image).into_iter().map(|r| (r, 0f64)).collect::<Vec<_>>()
    }
}

impl CascadeClassifier {
    /// Creates a cascade classifier, uninitialized. Before use, call load.
    pub fn new() -> CascadeClassifier {
        CascadeClassifier { inner: unsafe { opencv_cascade_classifier_new() } }
    }

    /// Creates a cascade classifier using the model specified.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let cc = CascadeClassifier::new();
        if cc.load(path) { Some(cc) } else { None }
    }

    /// Loads the classifier model from a path.
    pub fn load<P: AsRef<Path>>(&self, path: P) -> bool {
        let s = CString::new(path.as_ref()
                .to_str()
                .expect("only UTF-8 path is allowed"))
            .expect("failed to create CString to load cascade");
        unsafe { opencv_cascade_classifier_load(self.inner, (&s).as_ptr()) }
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
    pub fn detect_with_params(&self,
                              mat: &Mat,
                              scale_factor: f32,
                              min_neighbors: i32,
                              min_size: Size2i,
                              max_size: Size2i)
                              -> Vec<Rect> {
        let mut c_result = CVecOfRect::default();
        unsafe {
            opencv_cascade_classifier_detect(self.inner,
                                             mat.inner,
                                             &mut c_result,
                                             scale_factor as c_double,
                                             min_neighbors,
                                             0,
                                             min_size,
                                             max_size)
        }
        c_result.rustify()
    }
}

impl Drop for CascadeClassifier {
    fn drop(&mut self) {
        unsafe {
            opencv_cascade_classifier_drop(self.inner);
        }
    }
}

#[derive(Clone, Copy)]
enum CSvmDetector {}

///
#[derive(Debug)]
pub struct SvmDetector {
    inner: *mut CSvmDetector,
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
        SvmDetector { inner: unsafe { cv_hog_default_people_detector() } }
    }

    /// Returns the Daimler people detector.
    pub fn daimler_people_detector() -> SvmDetector {
        SvmDetector { inner: unsafe { cv_hog_daimler_people_detector() } }
    }
}

impl Drop for SvmDetector {
    fn drop(&mut self) {
        unsafe {
            cv_hog_detector_drop(self.inner);
        }
    }
}

/// Parameters that controls the behavior of HOG
#[derive(Debug, Clone, Copy)]
pub struct HogParams {
    /// Detection window size. Align to block size and block stride
    pub win_size: Size2i,

    /// Block size in pixels. Align to cell size. Only (16,16) is supported for now.
    pub block_size: Size2i,

    /// Block stride. It must be a multiple of cell size.
    pub block_stride: Size2i,

    /// Cell size. Only (8, 8) is supported for now.
    pub cell_size: Size2i,

    /// Number of bins. Only 9 bins per cell are supported for now.
    pub nbins: i32,

    /// Gaussian smoothing window parameter.
    pub win_sigma: f64,

    /// L2-Hys normalization method shrinkage.
    pub l2hys_threshold: f64,

    /// Flag to specify whether the gamma correction preprocessing is required or not.
    pub gamma_correction: bool,

    /// Maximum number of detection window increases.
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
    pub group_threshold: i32,
}

const DEFAULT_WIN_SIGMA: f64 = -1f64;
const DEFAULT_NLEVELS: usize = 64;

impl Default for HogParams {
    fn default() -> HogParams {

        let win_sigma = {
            if cfg!(feature = "gpu") {
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
            gamma_correction: true,
            nlevels: DEFAULT_NLEVELS,

            hit_threshold: 0f64,
            win_stride: Size2i::new(8, 8),
            padding: Size2i::default(),
            scale: 1.05,
            group_threshold: 2,
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
    fn cv_hog_detect(hog: *mut CHogDescriptor,
                     image: *mut CMat,
                     objs: *mut CVecOfRect,
                     weights: *mut CVecDouble,
                     win_stride: Size2i,
                     padding: Size2i,
                     scale: c_double,
                     final_threshold: c_double,
                     use_means_shift: bool);
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
        let mut detected = CVecOfRect::default();
        let mut weights = CVecDouble::default();
        unsafe {
            cv_hog_detect(self.inner,
                          image.inner,
                          &mut detected,
                          &mut weights,
                          self.params.win_stride,
                          self.params.padding,
                          self.params.scale,
                          2.0, // finalThreshold
                          false /* useMeanshiftGrouping */);
        }

        let results = detected.rustify();
        let weights = weights.rustify();
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
