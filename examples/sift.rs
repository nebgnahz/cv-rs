extern crate cv;
#[macro_use]
extern crate itertools;

use cv::features2d::*;
use cv::highgui::*;
use cv::imgcodecs::ImageReadMode;
use cv::*;
use itertools::Itertools;
use std::iter::once;
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

    let blue = mat.mix_channels(1, 1, [(0, 0)]);
    let green = mat.mix_channels(1, 1, [(1, 0)]);
    let red = mat.mix_channels(1, 1, [(2, 0)]);

    let mask = Mat::new();
    let sift: SIFT = SIFTBuilder::default().into();
    let (red_keypoints, _) = sift.detect_and_compute(&red, &mask);
    let (green_keypoints, _) = sift.detect_and_compute(&green, &mask);
    let (blue_keypoints, _) = sift.detect_and_compute(&blue, &mask);

    for (kps, (c1, c2)) in
        [red_keypoints, green_keypoints, blue_keypoints]
            .iter()
            .zip(&[(&green, &blue), (&blue, &red), (&green, &red)])
    {
        for kp in kps {
            let x = kp.point.x as c_int;
            let y = kp.point.y as c_int;
            c1.ellipse(
                Point2i::new(x, y),
                Size2i::new((kp.size * 1.0).ceil() as i32, (kp.size * 0.5).ceil() as i32),
                f64::from(kp.angle),
                0.0,
                360.0,
            );
            c2.ellipse(
                Point2i::new(x, y),
                Size2i::new((kp.size * 1.0).ceil() as i32, (kp.size * 0.5).ceil() as i32),
                f64::from(kp.angle),
                0.0,
                360.0,
            );
        }
    }
    let complete_buffer = izip!(
        blue.data().iter().cloned(),
        green.data().iter().cloned(),
        red.data().iter().cloned()
    )
    .flat_map(|(b, g, r)| once(b).chain(once(g)).chain(once(r)))
    .collect::<Vec<u8>>();
    let draw_mat = Mat::from_buffer(blue.rows, blue.cols, mat.cv_type(), &complete_buffer);

    draw_mat.show("SIFT Points", 0).unwrap();
}
