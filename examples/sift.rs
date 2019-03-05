extern crate cv;

use cv::features2d::*;
use cv::highgui::*;
use cv::imgcodecs::ImageReadMode;
use cv::*;
use std::f64::consts::PI;
use std::os::raw::c_int;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: sift ImageToLoadAndDisplay");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], ImageReadMode::Color).expect("Failed to read from path");

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (keypoints, _) = sift.detect_and_compute(&mat, &mask);

    let draw_mat = mat.clone();
    for kp in keypoints {
        let x = kp.point.x as c_int;
        let y = kp.point.y as c_int;
        draw_mat.ellipse(
            Point2i::new(x, y),
            Size2i::new((kp.size * 1.0).ceil() as i32, (kp.size * 0.5).ceil() as i32),
            f64::from(kp.angle),
            0.0,
            360.0,
        );
    }
    draw_mat.show("SIFT Points", 0).unwrap();
}
