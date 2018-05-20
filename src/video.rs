//! Video Analysis, see [OpenCV
//! video](http://docs.opencv.org/3.1.0/d7/de9/group__video.html)
pub mod tracking {
    //! Object Tracking, see [OpenCV video
    //! track](http://docs.opencv.org/3.1.0/dc/d6b/group__video__track.html)

    use core::*;
    use mat::*;
    use std::os::raw::c_int;

    // =========================================================================
    //   VideoTrack
    // =========================================================================
    enum CTermCriteria {}

    extern "C" {
        fn cv_term_criteria_new(t: TermType, count: c_int, epsilon: f64) -> *mut CTermCriteria;
        fn cv_term_criteria_drop(criteria: *mut CTermCriteria);
        fn cv_camshift(image: *mut CMat, w: Rect, c_criteria: *const CTermCriteria) -> RotatedRect;
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    /// Term criteria type, can be one of: Count, Eps or Count + Eps
    pub enum TermType {
        /// The maximum number of iterations or elements to compute
        Count = 1,

        /// the desired accuracy or change in parameters at which the iterative
        /// algorithm stops.
        EPS = 2,
    }

    /// Termination criteria for iterative algorithms.
    #[derive(Debug)]
    pub struct TermCriteria {
        c_criteria: *mut CTermCriteria,
    }

    impl TermCriteria {
        /// Creates a new termination criteria.
        pub fn new(t: TermType, max_count: c_int, epsilon: f64) -> Self {
            let c_criteria = unsafe { cv_term_criteria_new(t, max_count, epsilon) };
            TermCriteria { c_criteria: c_criteria }
        }
    }

    impl Drop for TermCriteria {
        fn drop(&mut self) {
            unsafe {
                cv_term_criteria_drop(self.c_criteria);
            }
        }
    }

    impl Mat {
        /// Finds an object center, size, and orientation; returns as `RotatedRect`.
        ///
        /// * `wndw` - initial search window.
        /// * `criteria` - stop criteria for the underlying meanShift.
        pub fn camshift(&self, wndw: Rect, criteria: &TermCriteria) -> RotatedRect {
            unsafe { cv_camshift(self.inner, wndw, criteria.c_criteria) }
        }
    }
}

pub mod analysis {
    //! Motion Analysis, see [OpenCV video
    //! motion](http://docs.cv.org/3.1.0/de/de1/group__video__motion.html)
}
