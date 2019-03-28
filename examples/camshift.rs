extern crate cv;
use cv::highgui::*;
use cv::imgproc::*;
use cv::videoio::*;
use cv::*;

use std::cell::RefCell;

struct SelectionStatus {
    selection: Rect,
    status: bool,
}

fn main() {
    let ss = RefCell::new(SelectionStatus {
        selection: Rect::default(),
        status: false,
    });

    let cap = VideoCapture::new(0);
    assert!(cap.is_open());

    let mut window = Window::new("Window", WindowFlag::Autosize).unwrap();
    window.set_mouse_callback(|data| {
        let MouseCallbackData { event, point, .. } = data;
        let mut ss = ss.borrow_mut();
        match event {
            MouseEventType::LButtonDown => {
                ss.selection.x = point.x;
                ss.selection.y = point.y;
            }
            MouseEventType::LButtonUp => {
                ss.selection.width = point.x - ss.selection.x;
                ss.selection.height = point.y - ss.selection.y;

                if ss.selection.width > 0 && ss.selection.height > 0 {
                    ss.status = true;
                }
            }
            _ => {}
        }
    }).unwrap();

    let mut is_tracking = false;

    let mut hist = Mat::new();
    let hsize = [16];
    let hranges = [0_f32, 180_f32];
    let pranges = [hranges];
    let mut track_window = Rect::default();

    while let Some(mut m) = cap.read() {
        m.flip(FlipCode::YAxis);

        let hsv = m.cvt_color(ColorConversion::BGR2HSV);

        let ch = [(0, 0)];
        let hue = hsv.mix_channels(1, 1, &ch);
        let mask = hsv.in_range(Scalar::new(0, 30, 10, 0), Scalar::new(180, 256, 256, 0));
        let channels = [0];

        if ss.borrow().status {
            println!("Initialize tracking, setting up CAMShift search");
            let selection = ss.borrow().selection;
            let roi = hue.roi(selection);
            let maskroi = mask.roi(selection);

            let raw_hist = roi.calc_hist(&channels, &maskroi, &hsize, &pranges);
            hist = raw_hist.normalize(0.0, 255.0, NormType::MinMax);

            track_window = selection;
            m.rectangle(selection);
            ss.borrow_mut().status = false;
            is_tracking = true;
        }

        if is_tracking {
            let back_project = hue.calc_back_project(&channels, &hist, &pranges) & mask;
            let criteria = TermCriteria::new(TermType::Count, 10, 1.0);
            let track_box = back_project.camshift(track_window, &criteria);

            m.rectangle(track_box.bounding_rect());
        }

        window.show(&m, Some(30)).unwrap();
    }
}
