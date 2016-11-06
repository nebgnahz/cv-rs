use super::Mat;
use super::core::CMat;

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
        unsafe {
            Mat::new_with_cmat(cv_mat_from_gpu_mat(gpu_mat.inner))
        }
    }
}
