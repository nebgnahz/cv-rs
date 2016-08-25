//! highgui: high-level GUI
extern crate libc;
use libc::{c_char, c_int, c_void};
use std::ffi::CString;

extern "C" {
    fn opencv_named_window(name: *const c_char, flags: c_int);
    fn opencv_set_mouse_callback(name: *const c_char,
                                 on_mouse: extern "C" fn(e: i32,
                                                         x: i32,
                                                         y: i32,
                                                         f: i32,
                                                         data: *mut c_void),
                                 userdata: *mut c_void);
}

pub fn highgui_named_window(name: &str, flags: WindowFlags) {
    let s = CString::new(name).unwrap();
    unsafe {
        opencv_named_window((&s).as_ptr(), flags as i32);
    }
}

type MouseCallback = fn(i32, i32, i32, i32, *mut c_void);
type _MouseCallback = extern "C" fn(i32, i32, i32, i32, *mut c_void);

pub fn highgui_set_mouse_callback(name: &str,
                                  on_mouse: MouseCallback,
                                  user_data: *mut c_void) {
    struct CallbackWrapper {
        cb: Box<MouseCallback>,
        data: *mut c_void,
    }

    extern "C" fn _mouse_callback(e: i32,
                                  x: i32,
                                  y: i32,
                                  f: i32,
                                  ud: *mut c_void) {
        let box_wrapper = unsafe { Box::from_raw(ud as *mut CallbackWrapper) };
        let cb_wrapper: CallbackWrapper = *box_wrapper;
        let true_callback = *(cb_wrapper.cb);
        true_callback(e, x, y, f, cb_wrapper.data);
    }

    let cb_wrapper = CallbackWrapper {
        cb: Box::new(on_mouse),
        data: user_data,
    };

    let box_wrapper: Box<CallbackWrapper> = Box::new(cb_wrapper);
    let box_wrapper_raw = Box::into_raw(box_wrapper) as *mut c_void;

    let s = CString::new(name).unwrap();
    unsafe {
        opencv_set_mouse_callback((&s).as_ptr(),
                                  _mouse_callback,
                                  box_wrapper_raw);
    }
}

pub enum WindowFlags {
    WindowNormal = 0x00000000,
    WindowAutosize = 0x00000001,
    WindowOpengl = 0x00001000,
}

/// Mouse Events
pub enum MouseEventTypes {
    /// Indicates that the mouse has moved over the window.
    MouseMove = 0,
    /// Indicates that the left mouse button is pressed.
    LButtonDown = 1,
    /// Indicates that the right mouse button is pressed.
    RButtonDown = 2,
    /// Indicates that the middle mouse button is pressed.
    MButtonDown = 3,
    /// Indicates that left mouse button is released.
    LButtonUp = 4,
    /// Indicates that right mouse button is released.
    RButtonUp = 5,
    /// Indicates that middle mouse button is released.
    MButtonUp = 6,
    /// Indicates that left mouse button is double clicked.
    LButtonClick = 7,
    /// Indicates that right mouse button is double clicked.
    RButtonClick = 8,
    /// Indicates that middle mouse button is double clicked.
    MButtonClick = 9,
    /// Positive/negative means forward/backward scrolling.
    MouseWheel = 10,
    /// Positive/negative means right and left scrolling.
    MouseHWheel = 11,
}
