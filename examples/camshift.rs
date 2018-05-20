extern crate cv;
use cv::highgui::*;
use cv::imgproc::*;
use cv::video::tracking::*;
use cv::videoio::*;
use cv::*;

struct SelectionStatus {
    selection: Rect,
    status: bool,
}

fn on_mouse(event: MouseEventType, x: i32, y: i32, _: i32, data: MouseCallbackData) {
    match event {
        MouseEventType::LButtonDown => {
            let ss = data as *mut SelectionStatus;
            let mut selection = unsafe { &mut (*ss).selection };
            selection.x = x;
            selection.y = y;
        }
        MouseEventType::LButtonUp => {
            let ss = data as *mut SelectionStatus;
            let mut selection = unsafe { &mut (*ss).selection };
            let mut status = unsafe { &mut (*ss).status };
            selection.width = x - selection.x;
            selection.height = y - selection.y;

            if selection.width > 0 && selection.height > 0 {
                *status = true;
            }
        }
        _ => {}
    }
}

fn main() {
    let mut selection_status = SelectionStatus {
        selection: Rect::default(),
        status: false,
    };
    let ss_ptr = &mut selection_status as *mut SelectionStatus;

    let cap = VideoCapture::new(0);
    assert!(cap.is_open());

    highgui_named_window("Window", WindowFlag::Autosize).unwrap();
    highgui_set_mouse_callback("Window", on_mouse, ss_ptr as MouseCallbackData).unwrap();

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

        if selection_status.status {
            println!("Initialize tracking, setting up CAMShift search");
            let selection = selection_status.selection;
            let roi = hue.roi(selection);
            let maskroi = mask.roi(selection);

            let raw_hist = roi.calc_hist(&channels, &maskroi, &hsize, &pranges);
            hist = raw_hist.normalize(0.0, 255.0, NormType::MinMax);

            track_window = selection;
            m.rectangle(selection);
            selection_status.status = false;
            is_tracking = true;
        }

        if is_tracking {
            let back_project = hue.calc_back_project(&channels, &hist, &pranges) & mask;
            let criteria = TermCriteria::new(TermType::Count, 10, 1.0);
            let track_box = back_project.camshift(track_window, &criteria);

            m.rectangle(track_box.bounding_rect());
        }

        m.show("Window", 30).unwrap();
    }
}
