use libc::{size_t, c_int, c_double};
use super::core::*;
use super::objdetect::{SvmDetector, CSvmDetector, ObjectDetect, HogParams};

// Opaque data struct for C bindings
pub enum CGpuMat {}

#[derive(Debug)]
pub struct GpuMat {
    pub inner: *mut CGpuMat,
    pub cols: i32,
    pub rows: i32,
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
    pub fn default() -> GpuMat {
        GpuMat {
            inner: unsafe { cv_gpu_mat_default() },
            cols: 0,
            rows: 0,
            depth: 0,
        }
    }

    pub fn from_raw(inner: *mut CGpuMat) -> GpuMat {
        GpuMat {
            inner: inner,
            cols: 0,
            rows: 0,
            depth: 0,
        }
    }

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
        unsafe { Mat::new_with_cmat(cv_mat_from_gpu_mat(gpu_mat.inner)) }
    }
}

impl From<Mat> for GpuMat {
    fn from(mat: Mat) -> GpuMat {
        unsafe { GpuMat::from_raw(cv_gpu_mat_from_mat(mat.inner)) }
    }
}

// Opaque data struct for C bindings
pub enum CGpuHog {}

#[derive(Debug)]
pub struct GpuHog {
    pub inner: *mut CGpuHog,
    pub params: HogParams,
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
    fn cv_gpu_hog_detect(hog: *mut CGpuHog, mat: *mut CGpuMat, found: *mut CVecOfRect);

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
        let mut found = CVecOfRect::default();
        let mut gpu_mat = GpuMat::default();
        gpu_mat.upload(image);
        unsafe { cv_gpu_hog_detect(self.inner, gpu_mat.inner, &mut found) }

        found.rustify().into_iter().map(|r| (r, 0f64)).collect::<Vec<_>>()
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
        }
    }
}

impl GpuHog {
    pub fn new(win_size: Size2i, block_size: Size2i, block_stride: Size2i, cell_size: Size2i, nbins: i32) -> GpuHog {
        let inner = unsafe { cv_gpu_hog_new(win_size, block_size, block_stride, cell_size, nbins) };
        let mut params = HogParams::default();
        GpuHog::update_params(inner, &mut params);
        GpuHog {
            inner: inner,
            params: params,
        }
    }

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
        }
    }

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

    pub fn set_svm_detector(&mut self, detector: SvmDetector) {
        unsafe { cv_gpu_hog_set_detector(self.inner, detector.inner) }
    }

    pub fn detect(&mut self, mat: &GpuMat) -> Vec<Rect> {
        let mut found = CVecOfRect::default();
        unsafe {
            cv_gpu_hog_detect(self.inner, mat.inner, &mut found);
        }
        found.rustify()
    }
}

impl Drop for GpuHog {
    fn drop(&mut self) {
        unsafe { cv_gpu_hog_drop(self.inner) }
    }
}
