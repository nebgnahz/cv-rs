extern crate cv;

use cv::highgui::*;

fn main () {

    let image_mat
        = cv::Mat::from_path("assets/lenna.png", cv::imgcodecs::ImageReadMode::Grayscale);

    match image_mat {
        Ok(mat) => {
            let scharr_x = mat.scharr(mat.depth,0,1,1.0,0.0,cv::BorderType::Default);
            scharr_x.show("Display", 0).unwrap()
        },
        Err(e) => println!("error loading image header: {:?}", e),
    }

}
