use libc::{c_double, c_char, c_int};
use std::ffi::CString;
use std::path::Path;
use std::vec::Vec;
use super::core::*;

/// The opaque type for C
enum CCascadeClassifier {}

/// Cascade classifier class for object detection.
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

impl CascadeClassifier {
    /// Create a cascade classifier, uninitialized. Before use, call load.
    pub fn new() -> CascadeClassifier {
        CascadeClassifier { inner: unsafe { opencv_cascade_classifier_new() } }
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) -> bool {
        let s = CString::new(path.as_ref()
                .to_str()
                .expect("only UTF-8 path is allowed"))
            .expect("failed to create CString to load cascade");
        unsafe { opencv_cascade_classifier_load(self.inner, (&s).as_ptr()) }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let cc = CascadeClassifier::new();
        cc.load(path);
        cc
    }

    pub fn detect(&self, mat: &Mat) -> Vec<Rect> {
        self.detect_with_params(mat, 1.1, 3, Size2i::default(), Size2i::default())
    }

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
                                             mat.get_cmat(),
                                             &mut c_result,
                                             scale_factor as c_double,
                                             min_neighbors as c_int,
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

pub enum CSvmDetector {}
pub struct SvmDetector {
    pub inner: *mut CSvmDetector,
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

enum CHogDescriptor {}

/// `HogDescriptor` implements Histogram of Oriented Gradients.
pub struct HogDescriptor {
    inner: *mut CHogDescriptor,
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
                     scale: c_double);
}

impl HogDescriptor {
    pub fn new() -> HogDescriptor {
        HogDescriptor { inner: unsafe { cv_hog_new() } }
    }

    pub fn set_svm_detector(&mut self, detector: SvmDetector) {
        unsafe { cv_hog_set_svm_detector(self.inner, detector.inner) }
    }

    /// Detect the particular object inside this image.
    /// win_stride: the amount of pixels that the between two sliding window
    /// padding: helps with detecting people on the edge
    pub fn detect(&mut self, mat: &Mat, win_stride: Size2i, padding: Size2i, scale: f64) -> Vec<(Rect, f64)> {
        let mut detected = CVecOfRect::default();
        let mut weights = CVecDouble::default();
        unsafe {
            cv_hog_detect(self.inner,
                          mat.inner,
                          &mut detected,
                          &mut weights,
                          win_stride,
                          padding,
                          scale as c_double)
        }

        let results = detected.rustify();
        let weights = weights.rustify();
        results.into_iter().zip(weights).collect::<Vec<_>>()
    }
}

impl Drop for HogDescriptor {
    fn drop(&mut self) {
        unsafe { cv_hog_drop(self.inner) }
    }
}
