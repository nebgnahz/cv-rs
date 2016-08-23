extern crate rust_vision;
use rust_vision::*;
use std::ffi::CString;

struct SelectionStatus {
    selection: Rect,
    status: bool,
}

extern "C" fn on_mouse(e: i32, x: i32, y: i32, _: i32, data: *mut CVoid) {
    let event: MouseEventTypes = unsafe { std::mem::transmute(e as u8) };
    match event {
        MouseEventTypes::LButtonDown => {
            let ss = data as *mut SelectionStatus;
            let mut selection = unsafe { &mut (*ss).selection };
            selection.x = x;
            selection.y = y;
        },
        MouseEventTypes::LButtonUp => {
            let ss = data as *mut SelectionStatus;
            let mut selection = unsafe { &mut (*ss).selection };
            let mut status = unsafe { &mut (*ss).status };
            selection.width = x - selection.x;
            selection.height = y - selection.y;
            *status = true;
        },
        _ => {},
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

    let s = CString::new("Window").unwrap();
    unsafe {
        opencv_named_window((&s).as_ptr(), WindowFlags::WindowAutosize as i32);
        opencv_set_mouse_callback((&s).as_ptr(),
                                  on_mouse,
                                  ss_ptr as *mut CVoid);
    }

    let m = Mat::new();
    let mut is_tracking = false;

    loop {
        cap.read(&m);
        let hsv = m.cvt_color(ColorConversionCodes::BGR2HSV);

        let ch = [0, 0];
        let hue = hsv.mix_channels(1, 1, &ch[0] as *const i32, 1);
        let mask = hsv.in_range(Scalar::new(0, 30, 10, 0),
                                Scalar::new(180, 256, 256, 0));

        let mut hist = Mat::new();
        let hsize = 16;
        let hranges = [0 as f32, 180 as f32];
        let phranges: [*const f32; 1] = [ &hranges[0] as *const f32 ];

        let mut track_window = Rect::default();

        if selection_status.status {
            // Object has been selected by user, set up CAMShift search
            // properties once
            let selection = selection_status.selection;
            let roi = hue.roi(selection);
            let maskroi = mask.roi(selection);

            hist = roi.calc_hist(std::ptr::null(), maskroi, 1, &hsize,
                                 &phranges[0] as *const *const f32);
            hist = hist.normalize(0 as f64, 255 as f64,
                                  NormTypes::NormMinMax);

            println!("{:?}", selection);
            track_window = selection;
            m.rectangle(selection);
            selection_status.status = false;
            is_tracking = true;
        }

        if is_tracking {
            let mut back_project = hue.calc_back_project(
                std::ptr::null(), hist, &phranges[0] as *const *const f32);
            back_project.logic_and(mask);
            let term_criteria = TermCriteria::new(TermType::Count, 10, 1 as f64);
            let track_box = back_project.camshift(track_window, &term_criteria);
            println!("{:?}", track_box);
        }

        m.show("Window", 30);
    }
}
