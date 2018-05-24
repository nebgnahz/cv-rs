//! highgui: high-level GUI
use failure::Error;
use mat::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

extern "C" {
    fn cv_named_window(name: *const c_char, flags: WindowFlag);
    fn cv_destroy_window(name: *const c_char);
    fn cv_set_mouse_callback(
        name: *const c_char,
        on_mouse: extern "C" fn(e: MouseEventType, x: c_int, y: c_int, f: c_int, data: *mut c_void),
        userdata: *mut c_void,
    );
    fn cv_imshow(name: *const c_char, cmat: *mut CMat);
    fn cv_wait_key(delay_ms: c_int) -> c_int;
}

/// Create a window that can be used as a placeholder for images and
/// trackbars. All created windows are referred to by their names. If a window
/// with the same name already exists, the function does nothing.
pub fn highgui_named_window(name: &str, flags: WindowFlag) -> Result<(), Error> {
    let s = CString::new(name)?;
    unsafe {
        cv_named_window(s.as_ptr(), flags);
    }
    Ok(())
}

/// Destroy the specified window with the given name.
pub fn highgui_destroy_window(name: &str) {
    let s = CString::new(name).unwrap();
    unsafe {
        cv_destroy_window((&s).as_ptr());
    }
}

/// Pointer referring to the data used in MouseCallback
pub type MouseCallbackData = *mut c_void;

/// Callback function for mouse events, primarily used in
/// [highgui_set_mouse_callback](fn.highgui_set_mouse_callback.html)
pub type MouseCallback = fn(MouseEventType, c_int, c_int, c_int, MouseCallbackData);

/// Set mouse handler for the specified window (identified by name). A callback
/// handler should be provided and optional user_data can be passed around.
pub fn highgui_set_mouse_callback(name: &str, on_mouse: MouseCallback, user_data: *mut c_void) -> Result<(), Error> {
    struct CallbackWrapper {
        cb: Box<MouseCallback>,
        data: *mut c_void,
    }

    extern "C" fn _mouse_callback(e: MouseEventType, x: c_int, y: c_int, f: c_int, ud: *mut c_void) {
        let cb_wrapper = unsafe { ptr::read(ud as *mut CallbackWrapper) };
        let true_callback = *(cb_wrapper.cb);
        true_callback(e, x, y, f, cb_wrapper.data);
        mem::forget(cb_wrapper.cb);
    }

    let box_wrapper: Box<CallbackWrapper> = Box::new(CallbackWrapper {
        cb: Box::new(on_mouse),
        data: user_data,
    });
    let box_wrapper_raw = Box::into_raw(box_wrapper) as *mut c_void;

    let s = CString::new(name)?;
    unsafe {
        cv_set_mouse_callback(s.as_ptr(), _mouse_callback, box_wrapper_raw);
    }
    Ok(())
}

/// Flags for [highgui_named_window](fn.highgui_named_window.html). This only
/// supports a subset of all cv::WindowFlags because C/C++ allows enum with the
/// same value but Rust is stricter.
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WindowFlag {
    /// the window can be resized (no constraint) or switched to fullscreen.
    Normal = 0x00000000,
    /// the window is constrained by the image displayed.
    Autosize = 0x00000001,
    /// the window is with opengl support.
    Opengl = 0x00001000,
    /// the window can be resized arbitrarily (no ratio constraint).
    FreeRatio = 0x00000100,
}

/// Mouse Events
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MouseEventType {
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

/// Provides some highgui functionallity
pub trait Show {
    /// Calls out to highgui to show the image, the duration is specified by `delay`.
    fn show(&self, name: &str, delay: c_int) -> Result<(), Error>;
}

impl Show for Mat {
    fn show(&self, name: &str, delay: c_int) -> Result<(), Error> {
        let s = CString::new(name)?;
        unsafe {
            cv_imshow((&s).as_ptr(), self.inner);
            cv_wait_key(delay);
        }
        Ok(())
    }
}
