extern crate cv;

use cv::features2d::*;
use cv::highgui::*;
use cv::imgcodecs::ImageReadMode;
use cv::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: sift ImageToLoadAndDisplay");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], ImageReadMode::AnyColor).expect("Failed to read from path");

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (keypoints, _) = sift.detect_and_compute(&mat, &mask);
    let mut data = mat.data().to_vec();
    for kp in keypoints {
        let x = kp.point.x as usize;
        let y = kp.point.y as usize;
        data[mat.elem_size() * (x + y * mat.size().width as usize) + 1] = 255;
    }
    Mat::from_buffer(mat.rows, mat.cols, mat.cv_type(), &data)
        .show("SIFT Points", 0)
        .unwrap();
}
