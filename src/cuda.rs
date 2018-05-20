//! Bindings to OpenCV's classes and functions that exploits GPU/Cuda. See
//! [cv::cuda](http://docs.opencv.org/3.1.0/d1/d1a/namespacecv_1_1cuda.html)

use super::core::*;
use super::errors::*;
use super::objdetect::{CSvmDetector, HogParams, ObjectDetect, SvmDetector};
use failure::Error;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};
use std::path::Path;
use *;

/// Opaque data struct for C/C++ cv::cuda::GpuMat bindings
#[derive(Clone, Copy, Debug)]
pub enum CGpuMat {}

/// `GpuMat` data structure in rust, bound to an opaque type in C/C++.
#[derive(Debug)]
pub struct GpuMat {
    /// The pointer to the opaque C/C++ data structure
    pub(crate) inner: *mut CGpuMat,

    /// Number of columns
    pub cols: c_int,

    /// Number of rows
    pub rows: c_int,

    /// Depth of this mat
    pub depth: c_int,
}

extern "C" {
    fn cv_cuda_gpu_mat_default() -> *mut CGpuMat;
    fn cv_cuda_gpu_mat_drop(gpu_mat: *mut CGpuMat);
    fn cv_cuda_gpu_mat_upload(gpu_mat: *mut CGpuMat, cpu_mat: *const CMat);
    fn cv_mat_from_gpu_mat(gpu_mat: *mut CGpuMat) -> *mut CMat;
    fn cv_cuda_gpu_mat_from_mat(mat: *mut CMat) -> *mut CGpuMat;
}

impl GpuMat {
    /// Creates a default `GpuMat`.
    pub fn default() -> GpuMat {
        GpuMat {
            inner: unsafe { cv_cuda_gpu_mat_default() },
            cols: 0,
            rows: 0,
            depth: 0,
        }
    }

    /// Creates a `GpuMat` from raw pointer.
    pub(crate) fn from_raw(inner: *mut CGpuMat) -> GpuMat {
        GpuMat {
            inner: inner,
            cols: 0,
            rows: 0,
            depth: 0,
        }
    }

    /// Uploads a normal `Mat`
    pub fn upload(&mut self, mat: &Mat) {
        unsafe {
            cv_cuda_gpu_mat_upload(self.inner, mat.inner);
        }
    }
}

impl Drop for GpuMat {
    fn drop(&mut self) {
        unsafe {
            cv_cuda_gpu_mat_drop(self.inner);
        }
    }
}

impl From<GpuMat> for Mat {
    fn from(gpu_mat: GpuMat) -> Mat {
        unsafe { Mat::from_raw(cv_mat_from_gpu_mat(gpu_mat.inner)) }
    }
}

impl From<Mat> for GpuMat {
    fn from(mat: Mat) -> GpuMat {
        unsafe { GpuMat::from_raw(cv_cuda_gpu_mat_from_mat(mat.inner)) }
    }
}

/// Opaque data struct for C bindings
#[derive(Clone, Copy, Debug)]
pub enum CGpuHog {}

#[derive(Debug)]
/// Data structure that performs Histogram of Gradient (HOG).
pub struct GpuHog {
    inner: *mut CGpuHog,

    /// Hog parameters.
    pub params: HogParams,

    /// Should return detection scores
    pub return_score: bool,
}

/// We can safely send a mutable pointer to a different thread
unsafe impl Send for GpuHog {}

extern "C" {
    fn cv_cuda_hog_default() -> *mut CGpuHog;
    fn cv_cuda_hog_new(
        win_size: Size2i,
        block_size: Size2i,
        block_stride: Size2i,
        cell_size: Size2i,
        nbins: c_int,
    ) -> *mut CGpuHog;
    fn cv_cuda_hog_drop(hog: *mut CGpuHog);
    fn cv_cuda_hog_set_detector(hog: *mut CGpuHog, d: *const CSvmDetector);
    fn cv_cuda_hog_detect(hog: *mut CGpuHog, mat: *mut CGpuMat, found: *mut CVec<Rect>);
    fn cv_cuda_hog_detect_with_conf(
        hog: *mut CGpuHog,
        mat: *mut CGpuMat,
        found: *mut CVec<Rect>,
        conf: *mut CVec<c_double>,
    );

    fn cv_cuda_hog_set_gamma_correction(hog: *mut CGpuHog, gamma: bool);
    fn cv_cuda_hog_set_group_threshold(hog: *mut CGpuHog, group_threshold: c_int);
    fn cv_cuda_hog_set_hit_threshold(hog: *mut CGpuHog, hit_threshold: c_double);
    fn cv_cuda_hog_set_l2hys_threshold(hog: *mut CGpuHog, l2hys_threshold: c_double);
    fn cv_cuda_hog_set_num_levels(hog: *mut CGpuHog, num_levels: usize);
    fn cv_cuda_hog_set_scale_factor(hog: *mut CGpuHog, scale_factor: c_double);
    fn cv_cuda_hog_set_win_sigma(hog: *mut CGpuHog, win_sigma: c_double);
    fn cv_cuda_hog_set_win_stride(hog: *mut CGpuHog, win_stride: Size2i);

    fn cv_cuda_hog_get_gamma_correction(hog: *mut CGpuHog) -> bool;
    fn cv_cuda_hog_get_group_threshold(hog: *mut CGpuHog) -> c_int;
    fn cv_cuda_hog_get_hit_threshold(hog: *mut CGpuHog) -> c_double;
    fn cv_cuda_hog_get_l2hys_threshold(hog: *mut CGpuHog) -> c_double;
    fn cv_cuda_hog_get_num_levels(hog: *mut CGpuHog) -> usize;
    fn cv_cuda_hog_get_scale_factor(hog: *mut CGpuHog) -> c_double;
    fn cv_cuda_hog_get_win_sigma(hog: *mut CGpuHog) -> c_double;
    fn cv_cuda_hog_get_win_stride(hog: *mut CGpuHog) -> Size2i;
}

impl ObjectDetect for GpuHog {
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)> {
        let mut gpu_mat = GpuMat::default();
        gpu_mat.upload(image);
        if self.return_score {
            self._detect_with_confidence(&gpu_mat)
        } else {
            self._detect(&gpu_mat)
        }
    }
}

impl Default for GpuHog {
    fn default() -> GpuHog {
        let inner = unsafe { cv_cuda_hog_default() };
        let mut params = HogParams::default();
        GpuHog::update_params(inner, &mut params);
        GpuHog {
            inner: inner,
            params: params,
            return_score: false,
        }
    }
}

impl GpuHog {
    /// Creates a new GpuHog detector.
    pub fn new(win_size: Size2i, block_size: Size2i, block_stride: Size2i, cell_size: Size2i, nbins: c_int) -> GpuHog {
        let inner = unsafe { cv_cuda_hog_new(win_size, block_size, block_stride, cell_size, nbins) };
        let mut params = HogParams::default();
        GpuHog::update_params(inner, &mut params);
        GpuHog {
            inner: inner,
            params: params,
            return_score: false,
        }
    }

    /// Should or not return the detection score
    pub fn return_score(&mut self, should: bool) {
        self.return_score = should;
    }

    /// Creates a new GpuHog detector with parameters specified inside `params`.
    pub fn with_params(params: HogParams) -> GpuHog {
        let inner = unsafe {
            cv_cuda_hog_new(
                params.win_size,
                params.block_size,
                params.block_stride,
                params.cell_size,
                params.nbins,
            )
        };
        unsafe {
            cv_cuda_hog_set_gamma_correction(inner, params.gamma_correction);
            cv_cuda_hog_set_l2hys_threshold(inner, params.l2hys_threshold);
            cv_cuda_hog_set_num_levels(inner, params.nlevels);
            cv_cuda_hog_set_win_sigma(inner, params.win_sigma);

            cv_cuda_hog_set_win_stride(inner, params.win_stride);
            cv_cuda_hog_set_scale_factor(inner, params.scale);
            cv_cuda_hog_set_hit_threshold(inner, params.hit_threshold);
            cv_cuda_hog_set_group_threshold(inner, params.group_threshold);
        }
        GpuHog {
            inner: inner,
            params: params,
            return_score: false,
        }
    }

    /// Updates the parameter inside this GpuHog detector.
    fn update_params(inner: *mut CGpuHog, params: &mut HogParams) {
        params.gamma_correction = unsafe { cv_cuda_hog_get_gamma_correction(inner) };
        params.group_threshold = unsafe { cv_cuda_hog_get_group_threshold(inner) };
        params.hit_threshold = unsafe { cv_cuda_hog_get_hit_threshold(inner) };
        params.l2hys_threshold = unsafe { cv_cuda_hog_get_l2hys_threshold(inner) };
        params.nlevels = unsafe { cv_cuda_hog_get_num_levels(inner) };
        params.scale = unsafe { cv_cuda_hog_get_scale_factor(inner) };
        params.win_sigma = unsafe { cv_cuda_hog_get_win_sigma(inner) };
        params.win_stride = unsafe { cv_cuda_hog_get_win_stride(inner) };
    }

    /// Sets the SVM detector.
    pub fn set_svm_detector(&mut self, detector: SvmDetector) {
        unsafe { cv_cuda_hog_set_detector(self.inner, detector.inner) }
    }

    /// Detects according to the SVM detector specified.
    fn _detect(&self, mat: &GpuMat) -> Vec<(Rect, f64)> {
        let mut found = CVec::<Rect>::default();
        unsafe {
            cv_cuda_hog_detect(self.inner, mat.inner, &mut found);
        }
        found.unpack().into_iter().map(|r| (r, 0f64)).collect::<Vec<_>>()
    }

    /// Detects and returns the results with confidence (scores)
    fn _detect_with_confidence(&self, mat: &GpuMat) -> Vec<(Rect, f64)> {
        let mut found = CVec::<Rect>::default();
        let mut conf = CVec::<c_double>::default();
        unsafe { cv_cuda_hog_detect_with_conf(self.inner, mat.inner, &mut found, &mut conf) }

        found
            .unpack()
            .into_iter()
            .zip(conf.unpack().into_iter())
            .collect::<Vec<_>>()
    }
}

impl Drop for GpuHog {
    fn drop(&mut self) {
        unsafe { cv_cuda_hog_drop(self.inner) }
    }
}

/// Opaque data struct for C bindings
#[derive(Clone, Copy, Debug)]
pub enum CGpuCascade {}

#[derive(Debug)]
/// Data structure that performs object detection with a cascade classifier.
pub struct GpuCascade {
    inner: *mut CGpuCascade,
}

/// We can safely send a mutable pointer to a different thread
unsafe impl Send for GpuCascade {}

extern "C" {
    fn cv_cuda_cascade_new(filename: *const c_char) -> *mut CGpuCascade;
    fn cv_cuda_cascade_drop(cascade: *mut CGpuCascade);
    fn cv_cuda_cascade_detect(cascade: *mut CGpuCascade, image: *const CGpuMat, objects: *mut CVec<Rect>);

    fn cv_cuda_cascade_set_find_largest_object(cascade: *mut CGpuCascade, value: bool);
    fn cv_cuda_cascade_set_max_num_objects(cascade: *mut CGpuCascade, max: c_int);
    fn cv_cuda_cascade_set_min_neighbors(cascade: *mut CGpuCascade, min: c_int);
    fn cv_cuda_cascade_set_max_object_size(cascade: *mut CGpuCascade, max: Size2i);
    fn cv_cuda_cascade_set_min_object_size(cascade: *mut CGpuCascade, min: Size2i);
    fn cv_cuda_cascade_set_scale_factor(cascade: *mut CGpuCascade, factor: c_double);

    fn cv_cuda_cascade_get_classifier_size(cascade: *const CGpuCascade) -> Size2i;
    fn cv_cuda_cascade_get_find_largest_object(cascade: *const CGpuCascade) -> bool;
    fn cv_cuda_cascade_get_max_num_objects(cascade: *const CGpuCascade) -> c_int;
    fn cv_cuda_cascade_get_min_neighbors(cascade: *const CGpuCascade) -> c_int;
    fn cv_cuda_cascade_get_max_object_size(cascade: *const CGpuCascade) -> Size2i;
    fn cv_cuda_cascade_get_min_object_size(cascade: *const CGpuCascade) -> Size2i;
    fn cv_cuda_cascade_get_scale_factor(cascade: *const CGpuCascade) -> c_double;
}

impl GpuCascade {
    /// Loads the classifier from a file.
    ///
    /// Name of the file from which the classifier is loaded. Only the old
    /// haar classifier (trained by the haar training application) and NVIDIA's
    /// nvbin are supported for HAAR and only new type of OpenCV XML cascade
    /// supported for LBP. The working haar models can be found at
    /// opencv_folder/data/haarcascades_cuda/.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        if let Some(p) = path.as_ref().to_str() {
            let s = CString::new(p)?;
            let inner = unsafe { cv_cuda_cascade_new((&s).as_ptr()) };
            return Ok(GpuCascade { inner: inner });
        }
        Err(CvError::InvalidPath(path.as_ref().to_path_buf()).into())
    }

    /// Detects objects of different sizes in the input image.
    pub fn detect_multiscale(&self, mat: &GpuMat) -> Vec<Rect> {
        let mut found = CVec::<Rect>::default();
        unsafe {
            cv_cuda_cascade_detect(self.inner, mat.inner, &mut found);
        }
        found.unpack()
    }

    /// Sets whether or not to find the only largest object.
    pub fn set_find_largest_object(&mut self, value: bool) {
        unsafe {
            cv_cuda_cascade_set_find_largest_object(self.inner, value);
        }
    }

    /// Sets the maximum number of objects.
    pub fn set_max_num_objects(&mut self, max: c_int) {
        unsafe {
            cv_cuda_cascade_set_max_num_objects(self.inner, max);
        }
    }

    /// Sets minimal neighbors required for a detection to be valid.
    pub fn set_min_neighbors(&mut self, min: c_int) {
        unsafe {
            cv_cuda_cascade_set_min_neighbors(self.inner, min);
        }
    }

    /// Sets the maximun object size.
    pub fn set_max_object_size(&mut self, max: Size2i) {
        unsafe {
            cv_cuda_cascade_set_max_object_size(self.inner, max);
        }
    }

    /// Sets the minimal object size.
    pub fn set_min_object_size(&mut self, min: Size2i) {
        unsafe {
            cv_cuda_cascade_set_min_object_size(self.inner, min);
        }
    }

    /// Sets the scale factor used in multiscale detection.
    pub fn set_scale_factor(&mut self, factor: f64) {
        unsafe {
            cv_cuda_cascade_set_scale_factor(self.inner, factor);
        }
    }

    /// Returns the classifier size.
    pub fn get_classifier_size(&self) -> Size2i {
        unsafe { cv_cuda_cascade_get_classifier_size(self.inner) }
    }

    /// Returns if the CascadeClassifier will only return the largest object.
    pub fn get_find_largest_object_flag(&self) -> bool {
        unsafe { cv_cuda_cascade_get_find_largest_object(self.inner) }
    }

    /// Returns the allowed maximal number of detected objects.
    pub fn get_max_num_objects(&self) -> c_int {
        unsafe { cv_cuda_cascade_get_max_num_objects(self.inner) }
    }

    /// Returns the number of minimal neighbors required for a detection to be
    /// valid.
    pub fn get_min_neighbors(&self) -> c_int {
        unsafe { cv_cuda_cascade_get_min_neighbors(self.inner) }
    }

    /// Returns the maximum object size.
    pub fn get_max_object_size(&self) -> Size2i {
        unsafe { cv_cuda_cascade_get_max_object_size(self.inner) }
    }

    /// Returns the minimal object size.
    pub fn get_min_object_size(&self) -> Size2i {
        unsafe { cv_cuda_cascade_get_min_object_size(self.inner) }
    }

    /// Returns the scale factor.
    pub fn get_scale_factor(&self) -> f64 {
        unsafe { cv_cuda_cascade_get_scale_factor(self.inner) }
    }
}

impl ObjectDetect for GpuCascade {
    fn detect(&self, image: &Mat) -> Vec<(Rect, f64)> {
        let mut gpu_mat = GpuMat::default();
        gpu_mat.upload(image);
        self.detect_multiscale(&gpu_mat).into_iter().map(|r| (r, 0.0)).collect()
    }
}

impl Drop for GpuCascade {
    fn drop(&mut self) {
        unsafe { cv_cuda_cascade_drop(self.inner) }
    }
}
