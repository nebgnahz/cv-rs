//! Provide the type that encapsulates all the parameters of the MSER extraction algorithm
use core::*;
use std::os::raw::*;
use *;

/// Maximally stable extremal region extractor.
///
/// See [MserBuilder](struct.MserBuilder.html).
#[derive(Debug)]
pub struct Mser {
    value: *mut u8,
}

impl Mser {
    /// Detect MSER regions.
    pub fn detect_regions(&self, image: &Mat) -> (Vec<Vec<Point2i>>, Vec<Rect>) {
        let mut msers: native::cvsys_CVec<native::cvsys_CVec<native::cvsys_Point2i>> = unsafe { std::mem::zeroed() };
        let mut bboxes: native::cvsys_CVec<native::cvsys_Rect> = unsafe { std::mem::zeroed() };
        unsafe {
            native::cvsys_mser_detect_regions(self.value, image.inner, &mut msers, &mut bboxes);
        }
        let msers = msers
            .iter()
            .map(|inner| inner.iter().cloned().map(Into::into).collect())
            .collect();
        let bboxes = bboxes.iter().cloned().map(Into::into).collect();
        (msers, bboxes)
    }
}

impl Drop for Mser {
    fn drop(&mut self) {
        unsafe {
            native::cvsys_mser_drop(self.value);
        }
    }
}

impl Default for Mser {
    fn default() -> Self {
        MserBuilder::default().into()
    }
}

/// Builder that provides defaults for [Mser](struct.Mser.html).
///
/// ## Defaults
/// - delta: 5,
/// - min_area: 60,
/// - max_area: 14400,
/// - max_variation: 0.25,
/// - min_diversity: 0.2,
/// - max_evolution: 200,
/// - area_threshold: 1.01,
/// - min_margin: 0.003,
/// - edge_blur_size: 5,
#[derive(Debug, Copy, Clone)]
pub struct MserBuilder {
    delta: c_int,
    min_area: c_int,
    max_area: c_int,
    max_variation: f64,
    min_diversity: f64,
    max_evolution: c_int,
    area_threshold: f64,
    min_margin: f64,
    edge_blur_size: c_int,
}

impl MserBuilder {
    /// Replace current delta with specified value
    pub fn delta(mut self, value: i32) -> Self {
        self.delta = value;
        self
    }

    /// Replace current min_area with specified value
    pub fn min_area(mut self, value: i32) -> Self {
        self.min_area = value;
        self
    }

    /// Replace current max_area with specified value
    pub fn max_area(mut self, value: i32) -> Self {
        self.max_area = value;
        self
    }

    /// Replace current max_variation with specified value
    pub fn max_variation(mut self, value: f64) -> Self {
        self.max_variation = value;
        self
    }

    /// Replace current min_diversity with specified value
    pub fn min_diversity(mut self, value: f64) -> Self {
        self.min_diversity = value;
        self
    }

    /// Replace current max_evolution with specified value
    pub fn max_evolution(mut self, value: i32) -> Self {
        self.max_evolution = value;
        self
    }

    /// Replace current area_threshold with specified value
    pub fn area_threshold(mut self, value: f64) -> Self {
        self.area_threshold = value;
        self
    }

    /// Replace current min_margin with specified value
    pub fn min_margin(mut self, value: f64) -> Self {
        self.min_margin = value;
        self
    }

    /// Replace current edge_blur_size with specified value
    pub fn edge_blur_size(mut self, value: i32) -> Self {
        self.edge_blur_size = value;
        self
    }
}

impl Default for MserBuilder {
    fn default() -> Self {
        Self {
            delta: 5,
            min_area: 60,
            max_area: 14400,
            max_variation: 0.25,
            min_diversity: 0.2,
            max_evolution: 200,
            area_threshold: 1.01,
            min_margin: 0.003,
            edge_blur_size: 5,
        }
    }
}

impl Into<Mser> for MserBuilder {
    fn into(self) -> Mser {
        let value = unsafe {
            native::cvsys_mser_new(
                self.delta,
                self.min_area,
                self.max_area,
                self.max_variation,
                self.min_diversity,
                self.max_evolution,
                self.area_threshold,
                self.min_margin,
                self.edge_blur_size,
            )
        };
        Mser { value }
    }
}
