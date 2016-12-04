extern crate cv;
use cv::*;
use cv::highgui::*;
use cv::videoio::*;

fn main() {
    let cap = VideoCapture::from_path("/Users/benzh/Downloads/video.mp4");
    println!("{}", cap.get(CapProp::FrameCount).unwrap());
    println!("{}", cap.get(CapProp::Fps).unwrap());
    // println!("{}", cap.set(CapProp::FrameWidth, 640.0));
    // println!("{}", cap.set(CapProp::FrameHeight, 480.0));
    println!("{}",
             codec_name(cap.get(CapProp::Fourcc).unwrap() as i32).unwrap());
    assert!(cap.is_open());
    let num = 50;
    let start = ::std::time::Instant::now();
    for _i in 0..num {
        // Get a few frames to estimate FPS
        cap.read().unwrap();
    }
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
    let elapsed_sec = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

    let avg_time = elapsed_sec / (num as f64);
    let fps = 1.0 / avg_time;

    // Learn size and CvType
    let image = cap.read().unwrap();
    let size = image.size();
    let is_color = image.cv_type() == CvType::Cv8UC3;

    println!("fps: {}, size: {}x{}, color: {}",
             fps,
             size.width,
             size.height,
             is_color);

    let codec = fourcc('M', 'J', 'P', 'G');
    let writer = VideoWriter::new("test2.avi", codec, 24.0, size, is_color);

    highgui_named_window("Window", WindowFlags::WindowAutosize);

    while let Some(image) = cap.read() {
        writer.write(&image);
        if let Some(bw) = writer.get(VideoWriterProperty::FrameBytes) {
            println!("frame bytes: {}", bw);
        }
        // image.show("Window", 5);
    }
}
