// This resembles the OpenCV read image example code:
// http://docs.opencv.org/3.1.0/db/deb/tutorial_display_image.html
extern crate rust_vision;
use rust_vision::*;
use rust_vision::highgui::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: display_image ImageToLoadAndDisplay");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], 1);

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    highgui_named_window("Display window", WindowFlags::WindowAutosize);
    mat.show("Display window", 0);
}
