//! Video Analysis, see [OpenCV
//! video](http://docs.opencv.org/3.1.0/d7/de9/group__video.html)
pub mod tracking {
    //! Object Tracking, see [OpenCV video
    //! track](http://docs.opencv.org/3.1.0/dc/d6b/group__video__track.html)

    use core::*;
    use mat::*;

    // =========================================================================
    //   VideoTrack
    // =========================================================================

    extern "C" {
        fn cv_camshift(image: *mut CMat, w: Rect, c_criteria: *const CTermCriteria) -> RotatedRect;
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
