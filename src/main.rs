// This resembles the OpenCV read image example code:
// http://docs.opencv.org/3.1.0/db/deb/tutorial_display_image.html
extern crate rust_vision;
use rust_vision::*;
use rust_vision::cuda::GpuMat;

fn main() {
    let mat = Mat::from_path("./assets/lenna.png", 1);
    let mut gpu_mat = GpuMat::default();
    gpu_mat.upload(&mat);

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    highgui_named_window("Display window", WindowFlags::WindowAutosize);
    mat.show("Display window", 0);
}
