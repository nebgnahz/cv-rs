use super::core::*;
use super::objdetect::{SvmDetector, CSvmDetector};

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
}

extern "C" {
    fn cv_gpu_hog_default() -> *mut CGpuHog;
    fn cv_gpu_hog_drop(hog: *mut CGpuHog);
    fn cv_gpu_hog_set_detector(hog: *mut CGpuHog, d: *const CSvmDetector);
    fn cv_gpu_hog_detect(hog: *mut CGpuHog, mat: *mut CGpuMat, found: *mut CVecOfRect);
}

impl GpuHog {
    pub fn default() -> GpuHog {
        GpuHog { inner: unsafe { cv_gpu_hog_default() } }
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
