extern crate cv;

mod knn_tests {
    use cv::imgcodecs::ImageReadMode;
    use cv::{mat::Mat, video::analysis::BackgroundSubtractorKNN};

    #[test]
    fn test_create_background_subtractor_knn_default() {
        #[allow(unused_mut)]
        let mut _bgs = BackgroundSubtractorKNN::default();
    }

    #[test]
    fn test_create_background_subtractor_knn_apply() {
        #[allow(unused_mut)]
        let mut bgs = BackgroundSubtractorKNN::default();

        let image = Mat::from_path("assets/Histogram_Comparison_Source_0.png", ImageReadMode::Color).unwrap();
        bgs.apply(&image, -1.0);
    }

    #[test]
    fn test_create_background_subtractor_knn_background_image() {
        #[allow(unused_mut)]
        let mut bgs = BackgroundSubtractorKNN::default();

        let image = Mat::from_path("assets/Histogram_Comparison_Source_0.png", ImageReadMode::Color).unwrap();
        assert_eq!(200, image.rows);
        assert_eq!(200, image.cols);
        bgs.apply(&image, -1.0);

        let background_image = bgs.background_image().unwrap();

        println!(
            "channels: {}, rows: {}, cols: {}, depth: {}",
            background_image.channels, background_image.rows, background_image.cols, background_image.depth,
        );

        assert_eq!(3, background_image.channels);
        assert_eq!(200, background_image.rows);
        assert_eq!(200, background_image.cols);
        assert_eq!(0, background_image.depth);
    }

}

mod mog2_tests {
    use cv::imgcodecs::ImageReadMode;
    use cv::{mat::Mat, video::analysis::BackgroundSubtractorMOG2};

    #[test]
    fn test_create_background_subtractor_mog2_default() {
        #[allow(unused_mut)]
        let mut _bgs = BackgroundSubtractorMOG2::default();
    }

    #[test]
    fn test_create_background_subtractor_mog2_apply() {
        #[allow(unused_mut)]
        let mut bgs = BackgroundSubtractorMOG2::default();

        let image = Mat::from_path("assets/Histogram_Comparison_Source_0.png", ImageReadMode::Color).unwrap();
        bgs.apply(&image, -1.0);
    }

    #[test]
    fn test_create_background_subtractor_mog2_background_image() {
        #[allow(unused_mut)]
        let mut bgs = BackgroundSubtractorMOG2::default();

        let image = Mat::from_path("assets/Histogram_Comparison_Source_0.png", ImageReadMode::Color).unwrap();
        assert_eq!(200, image.rows);
        assert_eq!(200, image.cols);
        bgs.apply(&image, -1.0);

        let background_image = bgs.background_image().unwrap();

        println!(
            "channels: {}, rows: {}, cols: {}, depth: {}",
            background_image.channels, background_image.rows, background_image.cols, background_image.depth,
        );

        assert_eq!(3, background_image.channels);
        assert_eq!(200, background_image.rows);
        assert_eq!(200, background_image.cols);
        assert_eq!(0, background_image.depth);
    }

}
