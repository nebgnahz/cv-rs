// The following code closely resembles the equivalent C code capture live video
// #include "opencv2/opencv.hpp"
// using namespace cv;
// int main(int, char**)
// {
//     VideoCapture cap(0); // open the default camera
//     if(!cap.isOpened())  // check if we succeeded
//         return -1;
//     Mat edges;
//     namedWindow("edges",1);
//     for(;;)
//     {
//         Mat frame;
//         cap >> frame; // get a new frame from camera
//         cvtColor(frame, edges, COLOR_BGR2GRAY);
//         GaussianBlur(edges, edges, Size(7,7), 1.5, 1.5);
//         Canny(edges, edges, 0, 30, 3);
//         imshow("edges", edges);
//         if(waitKey(30) >= 0) break;
//     }
//     // the camera will be deinitialized automatically in VideoCapture
//     // destructor
//     return 0;
// }

extern crate rust_vision;
use rust_vision::*;

fn main() {
    let cap = VideoCapture::new(0);
    assert!(cap.is_open());
    let m = Mat::new();

    highgui_named_window("Window", WindowFlags::WindowAutosize);
    loop {
        cap.read(&m);
        m.show("Window", 30);
    }
}
