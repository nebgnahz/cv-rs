extern crate cv;

use cv::highgui::*;
use cv::imgcodecs::ImageReadMode;
use cv::imgproc::ColorConversion;
use cv::*;

fn main() {
    ////////////////////////////////
    //
    // 1. Read the image
    //
    ///////////////////////////////

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: calchist <Path to Image>");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], ImageReadMode::Color).expect("Failed to read from path");

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    let hsv = mat.cvt_color(ColorConversion::BGR2HSV);

    ////////////////////////////////
    //
    // 2. Calculate the histogram
    //    (demo multiple channels)
    //
    ///////////////////////////////

    let hbins = 30;
    let sbins = 32;
    let hist_size = [hbins, sbins];

    let hranges = [0.0, 180.0];
    let sranges = [0.0, 256.0];
    let ranges = [hranges, sranges];

    let channels = [0, 1];

    let hist = hsv.calc_hist(&channels, &Mat::new(), &hist_size, &ranges);

    ////////////////////////////////
    //
    // 3. Display the histogram
    //
    ///////////////////////////////

    let min_max = hist.min_max_loc(&Mat::new());
    let max_val = min_max.1 as f32;

    let scale = 10;
    let hist_image = Mat::with_size(sbins * scale, hbins * scale, CvType::Cv8UC3 as i32);

    for h in 0..hbins {
        for s in 0..sbins {
            let bin_val = hist.at2::<f32>(h, s);
            let intensity = (bin_val * 255.0 / max_val) as i32;
            let rect = Rect::new(h * scale + 1, s * scale + 1, scale - 1, scale - 1);

            hist_image.rectangle_custom(rect, Scalar::all(intensity), LineType::Filled as i32, LineType::Line8);
        }
    }

    highgui_named_window("Display window", WindowFlag::Normal).unwrap();
    hist_image.show("Histogram", 0).unwrap();
}
