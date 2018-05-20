//! Provide the type that encapsulates all the parameters of the MSER extraction algorithm
use core::*;
use std::os::raw::*;
use *;

enum CMSER {}

extern "C" {
    fn cv_mser_new(
        delta: c_int,
        min_area: c_int,
        max_area: c_int,
        max_variation: c_double,
        min_diversity: c_double,
        max_evolution: c_int,
        area_threshold: c_double,
        min_margin: c_double,
        edge_blur_size: c_int,
    ) -> *mut CMSER;
    fn cv_mser_drop(cmser: *mut CMSER);
    fn cv_mser_detect_regions(
        detector: *const CMSER,
        image: *const CMat,
        msers: *mut CVec<CVec<Point2i>>,
        bboxes: *mut CVec<Rect>,
    );
}

/// Maximally stable extremal region extractor.
#[derive(Debug)]
pub struct MSER {
    value: *mut CMSER,
}

impl MSER {
    /// Creates a new maximally stable extremal region extractor criteria.
    pub fn new(
        delta: c_int,
        min_area: c_int,
        max_area: c_int,
        max_variation: f64,
        min_diversity: f64,
        max_evolution: c_int,
        area_threshold: f64,
        min_margin: f64,
        edge_blur_size: c_int,
    ) -> Self {
        let mser = unsafe {
            cv_mser_new(
                delta,
                min_area,
                max_area,
                max_variation,
                min_diversity,
                max_evolution,
                area_threshold,
                min_margin,
                edge_blur_size,
            )
        };
        MSER { value: mser }
    }

    /// Detect MSER regions.
    pub fn detect_regions(&self, image: &Mat) -> (Vec<Vec<Point2i>>, Vec<Rect>) {
        let mut msers = CVec::<CVec<Point2i>>::default();
        let mut bboxes = CVec::<Rect>::default();
        unsafe {
            cv_mser_detect_regions(self.value, image.inner, &mut msers, &mut bboxes);
        }
        let msers = msers.unpack();
        let boxes = bboxes.unpack();
        (msers, boxes)
    }
}

impl Drop for MSER {
    fn drop(&mut self) {
        unsafe {
            cv_mser_drop(self.value);
        }
    }
}

/// Builder that provides defaults for MSER
#[derive(Debug, Copy, Clone, Default)]
pub struct MSERBuilder {
    delta: Option<c_int>,
    min_area: Option<c_int>,
    max_area: Option<c_int>,
    max_variation: Option<f64>,
    min_diversity: Option<f64>,
    max_evolution: Option<c_int>,
    area_threshold: Option<f64>,
    min_margin: Option<f64>,
    edge_blur_size: Option<c_int>,
}

impl MSERBuilder {
    /// Replace current delta with specified value
    pub fn delta(mut self, value: c_int) -> Self {
        self.delta = Some(value);
        self
    }

    /// Replace current min_area with specified value
    pub fn min_area(mut self, value: c_int) -> Self {
        self.min_area = Some(value);
        self
    }

    /// Replace current max_area with specified value
    pub fn max_area(mut self, value: c_int) -> Self {
        self.max_area = Some(value);
        self
    }

    /// Replace current max_variation with specified value
    pub fn max_variation(mut self, value: f64) -> Self {
        self.max_variation = Some(value);
        self
    }

    /// Replace current min_diversity with specified value
    pub fn min_diversity(mut self, value: f64) -> Self {
        self.min_diversity = Some(value);
        self
    }

    /// Replace current max_evolution with specified value
    pub fn max_evolution(mut self, value: c_int) -> Self {
        self.max_evolution = Some(value);
        self
    }

    /// Replace current area_threshold with specified value
    pub fn area_threshold(mut self, value: f64) -> Self {
        self.area_threshold = Some(value);
        self
    }

    /// Replace current min_margin with specified value
    pub fn min_margin(mut self, value: f64) -> Self {
        self.min_margin = Some(value);
        self
    }

    /// Replace current edge_blur_size with specified value
    pub fn edge_blur_size(mut self, value: c_int) -> Self {
        self.edge_blur_size = Some(value);
        self
    }
}

impl Into<MSER> for MSERBuilder {
    fn into(self) -> MSER {
        MSER::new(
            self.delta.unwrap_or(5),
            self.min_area.unwrap_or(60),
            self.max_area.unwrap_or(14400),
            self.max_variation.unwrap_or(0.25),
            self.min_diversity.unwrap_or(0.2),
            self.max_evolution.unwrap_or(200),
            self.area_threshold.unwrap_or(1.01),
            self.min_margin.unwrap_or(0.003),
            self.edge_blur_size.unwrap_or(5),
        )
    }
}
