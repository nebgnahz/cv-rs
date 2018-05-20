//! Provide the type that encapsulates all the parameters of the SURF extraction algorithm
use super::*;
use core::*;
use std::os::raw::*;
use *;

enum CSURF {}

extern "C" {
    fn cv_surf_new(
        hessian_threshold: c_double,
        octaves: c_int,
        octave_layers: c_int,
        extended: bool,
        upright: bool,
    ) -> *mut CSURF;
    fn cv_surf_drop(cmser: *mut CSURF);
    fn cv_surf_detect_and_compute(
        detector: *const CSURF,
        image: *const CMat,
        mask: *const CMat,
        keypoints: *mut CVec<KeyPoint>,
        descriptors: *mut CMat,
        use_provided_keypoints: bool,
    );
}

/// Speeded up robust features extractor.
#[derive(Debug)]
pub struct SURF {
    value: *mut CSURF,
}

impl SURF {
    /// Creates a new maximally stable extremal region extractor criteria.
    pub fn new(hessian_threshold: f64, octaves: c_int, octave_layers: c_int, extended: bool, upright: bool) -> Self {
        let surf = unsafe { cv_surf_new(hessian_threshold, octaves, octave_layers, extended, upright) };
        SURF { value: surf }
    }
}

impl Drop for SURF {
    fn drop(&mut self) {
        unsafe {
            cv_surf_drop(self.value);
        }
    }
}

/// Builder that provides defaults for MSER
#[derive(Debug, Copy, Clone, Default)]
pub struct SURFBuilder {
    hessian_threshold: Option<f64>,
    octaves: Option<c_int>,
    octave_layers: Option<c_int>,
    extended: Option<bool>,
    upright: Option<bool>,
}

impl SURFBuilder {
    /// Replace current octave_layers with specified value
    pub fn hessian_threshold(mut self, value: f64) -> Self {
        self.hessian_threshold = Some(value);
        self
    }

    /// Replace current octave_layers with specified value
    pub fn octaves(mut self, value: c_int) -> Self {
        self.octaves = Some(value);
        self
    }

    /// Replace current octave_layers with specified value
    pub fn octave_layers(mut self, value: c_int) -> Self {
        self.octave_layers = Some(value);
        self
    }

    /// Replace current extended with specified value
    pub fn extended(mut self, value: bool) -> Self {
        self.extended = Some(value);
        self
    }

    /// Replace current delta with specified value
    pub fn upright(mut self, value: bool) -> Self {
        self.upright = Some(value);
        self
    }
}

impl Into<SURF> for SURFBuilder {
    fn into(self) -> SURF {
        SURF::new(
            self.hessian_threshold.unwrap_or(100.0),
            self.octaves.unwrap_or(4),
            self.octave_layers.unwrap_or(3),
            self.extended.unwrap_or(false),
            self.upright.unwrap_or(false),
        )
    }
}

impl Feature2D for SURF {
    fn detect_and_compute(&self, image: &Mat, mask: &Mat) -> (Vec<KeyPoint>, Mat) {
        let mut keypoints = CVec::<KeyPoint>::default();
        let descriptors = CMat::new();
        unsafe {
            cv_surf_detect_and_compute(self.value, image.inner, mask.inner, &mut keypoints, descriptors, false);
        }
        (keypoints.unpack(), Mat::from_raw(descriptors))
    }
}
