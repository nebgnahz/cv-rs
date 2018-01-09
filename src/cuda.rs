//! Bindings to OpenCV's classes and functions that exploits GPU/Cuda. See
//! [cv::cuda](http://docs.opencv.org/3.1.0/d1/d1a/namespacecv_1_1cuda.html)

use libc::{size_t, c_int, c_double};
use super::core::*;
use super::objdetect::{SvmDetector, CSvmDetector, ObjectDetect, HogParams};

/// Opaque data struct for C/C++ cv::cuda::GpuMat bindings
#[derive(Clone, Copy, Debug)]
pub enum CGpuMat {}

/// `GpuMat` data structure in rust, bound to an opaque type in C/C++.
#[derive(Debug)]
pub struct GpuMat {
    /// The pointer to the opaque C/C++ data structure
    pub inner: *mut CGpuMat,

    /// Number of columns
    pub cols: i32,

    /// Number of rows
    pub rows: i32,

    /// Depth of this mat
    pub depth: i32,
}

extern "C" {
    fn cv_gpu_mat_default() -> *mut CGpuMat;
    fn cv_gpu_mat_drop(gpu_mat: *mut CGpuMat);
    fn cv_gpu_mat_upload(gpu_mat: *mut CGpuMat, cpu_mat: *const CMat);
    fn cv_mat_from_gpu_mat(gpu_mat: *mut CGpuMat) -> *mut CMat;
    fn cv_gpu_mat_from_mat(mat: *mut CMat) -> *mut CGpuMat;
}

impl GpuMat {
    /// Creates a default `GpuMat`.
    pub fn default() -> GpuMat {
        GpuMat {
            inner: unsafe { cv_gpu_mat_default() },
            cols: 0,
            rows: 0,
            depth: 0,
        }
    }

    /// Creates a `GpuMat` from raw pointer.
    pub fn from_raw(inner: *mut CGpuMat) -> GpuMat {
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
            cv_gpu_mat_upload(self.inner, mat.inner);
        }
    }
}

impl Drop for GpuMat {
    fn drop(&mut self) {
        unsafe {
            cv_gpu_mat_drop(self.inner);
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
        unsafe { GpuMat::from_raw(cv_gpu_mat_from_mat(mat.inner)) }
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

extern "C" {
    fn cv_gpu_hog_default() -> *mut CGpuHog;
    fn cv_gpu_hog_new(win_size: Size2i,
                      block_size: Size2i,
                      block_stride: Size2i,
                      cell_size: Size2i,
                      nbins: i32)
                      -> *mut CGpuHog;
    fn cv_gpu_hog_drop(hog: *mut CGpuHog);
    fn cv_gpu_hog_set_detector(hog: *mut CGpuHog, d: *const CSvmDetector);
    fn cv_gpu_hog_detect(hog: *mut CGpuHog, mat: *mut CGpuMat, found: *mut CVec<Rect>);
    fn cv_gpu_hog_detect_with_conf(hog: *mut CGpuHog, mat: *mut CGpuMat, found: *mut CVec<Rect>, conf: *mut CVec<c_double>);

    fn cv_gpu_hog_set_gamma_correction(hog: *mut CGpuHog, gamma: bool);
    fn cv_gpu_hog_set_group_threshold(hog: *mut CGpuHog, group_threshold: c_int);
    fn cv_gpu_hog_set_hit_threshold(hog: *mut CGpuHog, hit_threshold: c_double);
    fn cv_gpu_hog_set_l2hys_threshold(hog: *mut CGpuHog, l2hys_threshold: c_double);
    fn cv_gpu_hog_set_num_levels(hog: *mut CGpuHog, num_levels: size_t);
    fn cv_gpu_hog_set_scale_factor(hog: *mut CGpuHog, scale_factor: c_double);
    fn cv_gpu_hog_set_win_sigma(hog: *mut CGpuHog, win_sigma: c_double);
    fn cv_gpu_hog_set_win_stride(hog: *mut CGpuHog, win_stride: Size2i);

    fn cv_gpu_hog_get_gamma_correction(hog: *mut CGpuHog) -> bool;
    fn cv_gpu_hog_get_group_threshold(hog: *mut CGpuHog) -> c_int;
    fn cv_gpu_hog_get_hit_threshold(hog: *mut CGpuHog) -> c_double;
    fn cv_gpu_hog_get_l2hys_threshold(hog: *mut CGpuHog) -> c_double;
    fn cv_gpu_hog_get_num_levels(hog: *mut CGpuHog) -> size_t;
    fn cv_gpu_hog_get_scale_factor(hog: *mut CGpuHog) -> c_double;
    fn cv_gpu_hog_get_win_sigma(hog: *mut CGpuHog) -> c_double;
    fn cv_gpu_hog_get_win_stride(hog: *mut CGpuHog) -> Size2i;
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
        let inner = unsafe { cv_gpu_hog_default() };
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
    pub fn new(win_size: Size2i,
               block_size: Size2i,
               block_stride: Size2i,
               cell_size: Size2i,
               nbins: i32)
               -> GpuHog {
        let inner = unsafe { cv_gpu_hog_new(win_size, block_size, block_stride, cell_size, nbins) };
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
            cv_gpu_hog_new(params.win_size,
                           params.block_size,
                           params.block_stride,
                           params.cell_size,
                           params.nbins)
        };
        unsafe {
            cv_gpu_hog_set_gamma_correction(inner, params.gamma_correction);
            cv_gpu_hog_set_l2hys_threshold(inner, params.l2hys_threshold);
            cv_gpu_hog_set_num_levels(inner, params.nlevels);
            cv_gpu_hog_set_win_sigma(inner, params.win_sigma);

            cv_gpu_hog_set_win_stride(inner, params.win_stride);
            cv_gpu_hog_set_scale_factor(inner, params.scale);
            cv_gpu_hog_set_hit_threshold(inner, params.hit_threshold);
            cv_gpu_hog_set_group_threshold(inner, params.group_threshold);
        }
        GpuHog {
            inner: inner,
            params: params,
            return_score: false,
        }
    }

    /// Updates the parameter inside this GpuHog detector.
    fn update_params(inner: *mut CGpuHog, params: &mut HogParams) {
        params.gamma_correction = unsafe { cv_gpu_hog_get_gamma_correction(inner) };
        params.group_threshold = unsafe { cv_gpu_hog_get_group_threshold(inner) };
        params.hit_threshold = unsafe { cv_gpu_hog_get_hit_threshold(inner) };
        params.l2hys_threshold = unsafe { cv_gpu_hog_get_l2hys_threshold(inner) };
        params.nlevels = unsafe { cv_gpu_hog_get_num_levels(inner) };
        params.scale = unsafe { cv_gpu_hog_get_scale_factor(inner) };
        params.win_sigma = unsafe { cv_gpu_hog_get_win_sigma(inner) };
        params.win_stride = unsafe { cv_gpu_hog_get_win_stride(inner) };
    }

    /// Sets the SVM detector.
    pub fn set_svm_detector(&mut self, detector: SvmDetector) {
        unsafe { cv_gpu_hog_set_detector(self.inner, detector.inner) }
    }

    /// Detects according to the SVM detector specified.
    fn _detect(&self, mat: &GpuMat) -> Vec<(Rect, f64)> {
        let mut found = CVec<Rect>::default();
        unsafe {
            cv_gpu_hog_detect(self.inner, mat.inner, &mut found);
        }
        found.unpack().into_iter().map(|r| (r, 0f64)).collect::<Vec<_>>()
    }

    /// Detects and returns the results with confidence (scores)
    fn _detect_with_confidence(&self, mat: &GpuMat) -> Vec<(Rect, f64)> {
        let mut found = CVec<Rect>::default();
        let mut conf = CVec<c_double>::default();
        unsafe { cv_gpu_hog_detect_with_conf(self.inner, mat.inner, &mut found, &mut conf) }

        found.unpack().into_iter().zip(conf.unpack().into_iter()).collect::<Vec<_>>()
    }
}

impl Drop for GpuHog {
    fn drop(&mut self) {
        unsafe { cv_gpu_hog_drop(self.inner) }
    }
}
