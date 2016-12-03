extern crate rust_vision;
use rust_vision::*;
use rust_vision::videoio::*;

fn main() {
    let cap = VideoCapture::new(0);
    println!("{}", cap.set(CapProp::FrameCount, 20.0));
    println!("{}", cap.set(CapProp::FrameWidth, 640.0));
    println!("{}", cap.set(CapProp::FrameHeight, 480.0));
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

    let codec = fourcc('M', 'P', '4', 'V');
    let writer = VideoWriter::new("test.avi", codec, fps, size, is_color);

    highgui_named_window("Window", WindowFlags::WindowAutosize);

    loop {
        let image = cap.read().unwrap();
        writer.write(&image);
        if let Some(bw) = writer.get(VideoWriterProperty::FrameBytes) {
            println!("frame bytes: {}", bw);
        }
        image.show("Window", 5);
    }
}
