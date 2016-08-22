extern crate rust_vision;
use rust_vision::*;
use std::ffi::CString;

struct SelectionStatus {
    selection: Rect,
    status: bool,
}

extern "C" fn on_mouse(e: i32, x: i32, y: i32, data: *mut CVoid) {
    let ss = data as *mut SelectionStatus;
    let mut selection = unsafe { &mut (*ss).selection };
    let mut status = unsafe { &mut (*ss).status };

    let event: MouseEventTypes = unsafe { std::mem::transmute(e as u8) };
    match event {
        MouseEventTypes::LButtonDown => {
            selection.x = x;
            selection.y = y;
        },
        MouseEventTypes::LButtonUp => {
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
    loop {
        cap.read(&m);
        let hsv = m.cvt_color(ColorConversionCodes::BGR2HSV);

        if selection_status.status {
            println!("{:?}", selection_status.selection);
        }

        hsv.show("Window", 30);
    }
}
