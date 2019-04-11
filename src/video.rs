//! Video Analysis, see [OpenCV
//! video](http://docs.opencv.org/3.1.0/d7/de9/group__video.html)
pub mod tracking {
    //! Object Tracking, see [OpenCV video
    //! track](http://docs.opencv.org/3.1.0/dc/d6b/group__video__track.html)

    use core::{CTermCriteria, Rect, RotatedRect, TermCriteria};
    use mat::{CMat, Mat};

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

    pub use self::background_subtractor_knn::BackgroundSubtractorKNN;
    pub use self::background_subtractor_mog2::BackgroundSubtractorMOG2;

    mod background_subtractor_knn {
        use mat::{CMat, Mat};

        extern "C" {
            fn cv_create_background_subtractor_knn(
                history: i64,
                dist2threshold: f64,
                detect_shadows: bool,
            ) -> *mut CPtrBackgroundSubtractorKNN;

            fn cv_create_background_subtractor_knn_default() -> *mut CPtrBackgroundSubtractorKNN;

            fn cv_background_subtractor_knn_drop(background_subtractor: *mut CPtrBackgroundSubtractorKNN);

            fn cv_background_subtractor_knn_apply(
                background_subtractor: *mut CPtrBackgroundSubtractorKNN,
                input_image: *const CMat,
                output_foreground: *mut CMat,
                learning_rate: f64,
            );

            fn cv_background_subtractor_knn_get_background_image(
                background_subtractor: *mut CPtrBackgroundSubtractorKNN,
                output_background: *mut CMat,
            );
        }

        pub(crate) enum CPtrBackgroundSubtractorKNN {}

        #[derive(Clone, Debug)]
        /// A KNN Background Subtractor algorithm.
        pub struct BackgroundSubtractorKNN {
            inner: *mut CPtrBackgroundSubtractorKNN,
            exposed_to_image: bool,
        }

        impl BackgroundSubtractorKNN {
            /// Makes a new BackgroundSubtractorKNN.
            pub fn new(history: i64, dist2threshold: f64, detect_shadows: bool) -> Self {
                let bgs = unsafe { cv_create_background_subtractor_knn(history, dist2threshold, detect_shadows) };

                BackgroundSubtractorKNN {
                    inner: bgs,
                    exposed_to_image: false,
                }
            }

            /// Computes the next foreground mask from input image.
            pub fn apply(&mut self, image: &Mat, learning_rate: f64) -> Mat {
                #[allow(unused_mut)]
                let mut c_foreground = CMat::new();
                unsafe {
                    cv_background_subtractor_knn_apply(self.inner, image.inner, c_foreground, learning_rate);
                };
                self.exposed_to_image = true;

                Mat::from_raw(c_foreground)
            }

            /// Return the background image.
            pub fn background_image(self) -> Option<Mat> {
                if self.exposed_to_image {
                    #[allow(unused_mut)]
                    let mut c_background = CMat::new();

                    unsafe {
                        cv_background_subtractor_knn_get_background_image(self.inner, c_background);
                    };

                    Some(Mat::from_raw(c_background))
                } else {
                    None
                }
            }
        }

        impl Default for BackgroundSubtractorKNN {
            fn default() -> Self {
                let bgs = unsafe { cv_create_background_subtractor_knn_default() };

                BackgroundSubtractorKNN {
                    inner: bgs,
                    exposed_to_image: false,
                }
            }
        }
        impl Drop for BackgroundSubtractorKNN {
            fn drop(&mut self) {
                unsafe { cv_background_subtractor_knn_drop(self.inner) }
            }
        }
    }

    mod background_subtractor_mog2 {
        use mat::{CMat, Mat};

        extern "C" {
            fn cv_create_background_subtractor_mog2(
                history: i64,
                dist2threshold: f64,
                detect_shadows: bool,
            ) -> *mut CPtrBackgroundSubtractorMOG2;

            fn cv_create_background_subtractor_mog2_default() -> *mut CPtrBackgroundSubtractorMOG2;

            fn cv_background_subtractor_mog2_drop(background_subtractor: *mut CPtrBackgroundSubtractorMOG2);

            fn cv_background_subtractor_mog2_apply(
                background_subtractor: *mut CPtrBackgroundSubtractorMOG2,
                input_image: *const CMat,
                output_foreground: *mut CMat,
                learning_rate: f64,
            );

            fn cv_background_subtractor_mog2_get_background_image(
                background_subtractor: *mut CPtrBackgroundSubtractorMOG2,
                output_background: *mut CMat,
            );
        }

        pub(crate) enum CPtrBackgroundSubtractorMOG2 {}

        #[derive(Clone, Debug)]
        /// A MOG2 Background Subtractor algorithm.
        pub struct BackgroundSubtractorMOG2 {
            pub(crate) inner: *mut CPtrBackgroundSubtractorMOG2,
            exposed_to_image: bool,
        }

        impl BackgroundSubtractorMOG2 {
            /// Makes a new BackgroundSubtractorMOG2.
            pub fn new(history: i64, dist2threshold: f64, detect_shadows: bool) -> Self {
                let background_subtractor =
                    unsafe { cv_create_background_subtractor_mog2(history, dist2threshold, detect_shadows) };

                BackgroundSubtractorMOG2 {
                    inner: background_subtractor,
                    exposed_to_image: false,
                }
            }

            /// Computes the next foreground mask from input image.
            pub fn apply(&mut self, image: &Mat, learning_rate: f64) -> Mat {
                #[allow(unused_mut)]
                let mut c_foreground = CMat::new();

                unsafe {
                    cv_background_subtractor_mog2_apply(self.inner, image.inner, c_foreground, learning_rate);
                }
                self.exposed_to_image = true;

                Mat::from_raw(c_foreground)
            }

            /// Return the background image.
            pub fn background_image(self) -> Option<Mat> {
                if self.exposed_to_image {
                    #[allow(unused_mut)]
                    let mut c_background = CMat::new();

                    unsafe {
                        cv_background_subtractor_mog2_get_background_image(self.inner, c_background);
                    }
                    Some(Mat::from_raw(c_background))
                } else {
                    None
                }
            }
        }

        impl Default for BackgroundSubtractorMOG2 {
            fn default() -> Self {
                let background_subtractor = unsafe { cv_create_background_subtractor_mog2_default() };

                BackgroundSubtractorMOG2 {
                    inner: background_subtractor,
                    exposed_to_image: false,
                }
            }
        }
        impl Drop for BackgroundSubtractorMOG2 {
            fn drop(&mut self) {
                unsafe { cv_background_subtractor_mog2_drop(self.inner) }
            }
        }
    }
}
