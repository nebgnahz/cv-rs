//! highgui: high-level GUI
use failure::Error;
use mat::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;
use *;

use std::fmt;

/// Create a window that you can show data in and allow the user to manipulate it.
pub struct Window {
    name: String,
    callback: Option<Box<CallbackWrapper>>,
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window {{ name: {} }}", self.name)
    }
}

struct CallbackWrapper {
    callback: Box<dyn FnMut(MouseCallbackData)>,
}

impl Window {
    /// Create a window that can be used as a placeholder for images and
    /// trackbars. All created windows are referred to by their names. If a window
    /// with the same name already exists, the function does nothing.
    pub fn new(name: &str, flags: WindowFlag) -> Result<Window, Error> {
        let s = CString::new(name)?;
        unsafe {
            native::cvsys_nat_named_window(s.as_ptr(), flags as i32);
        }
        Ok(Window{
            name: name.to_owned(),
            callback: None,
        })
    }

    /// Shows the data in the window until a key is pressed.
    /// If delay is `None` the window will stay open indefinitely.
    /// Returns the key that was pressed, if a key was pressed.
    pub fn show<S: Show>(&self, s: &S, delay: Option<u32>) -> Result<Option<u32>, Error> {
        s.show(&self.name, delay)
    }

    /// Set mouse handler for the specified window (identified by name). A callback
    /// handler should be provided and optional user_data can be passed around.
    pub fn set_mouse_callback<'a, F: FnMut(MouseCallbackData) + 'a>(&'a mut self, on_mouse: F) -> Result<(), Error> {
        extern "C" fn _mouse_callback(e: c_int, x: c_int, y: c_int, flags: c_int, ud: *mut c_void) {
            let cb_wrapper = unsafe { &mut *(ud as *mut CallbackWrapper) };
            (cb_wrapper.callback)(MouseCallbackData{
                event: e.into(),
                point: Point2i { x, y, },
                flags: flags as u32,
            });
        }

        self.callback = Some(Box::new(CallbackWrapper {
            callback: unsafe { std::mem::transmute(Box::<F>::new( on_mouse ) as Box<dyn FnMut(MouseCallbackData) + 'a>) },
        }));

        let callback_box: &mut Box<CallbackWrapper> = self.callback.as_mut().unwrap();
        let callback_ref: &mut CallbackWrapper = &mut **callback_box;
        let callback_pointer = callback_ref as *mut CallbackWrapper as *mut c_void;

        let s = CString::new(self.name.as_str())?;
        unsafe {
            native::cvsys_nat_set_mouse_callback(s.as_ptr(), Some(_mouse_callback), callback_pointer);
        }
        Ok(())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let s = CString::new(self.name.as_str()).unwrap();
        unsafe {
            native::cvsys_nat_destroy_window((&s).as_ptr());
        }
    }
}

/// See [the OpenCV documentation](https://docs.opencv.org/3.4.1/d7/dfc/group__highgui.html).
#[derive(Copy, Clone, Debug)]
pub struct MouseCallbackData {
    /// https://docs.opencv.org/3.4.1/d7/dfc/group__highgui.html
    pub event: MouseEventType,
    /// The mouse position.
    pub point: Point2i,
    /// https://docs.opencv.org/3.4.1/d7/dfc/group__highgui.html#gaab4dc057947f70058c80626c9f1c25ce
    pub flags: u32,
}

/// Flags for [highgui_named_window](fn.highgui_named_window.html). This only
/// supports a subset of all cv::WindowFlags because C/C++ allows enum with the
/// same value but Rust is stricter.
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WindowFlag {
    /// the window can be resized (no constraint) or switched to fullscreen.
    Normal = 0x0000_0000,
    /// the window is constrained by the image displayed.
    Autosize = 0x0000_0001,
    /// the window is with opengl support.
    Opengl = 0x0000_1000,
    /// the window can be resized arbitrarily (no ratio constraint).
    FreeRatio = 0x0000_0100,
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

impl From<c_int> for MouseEventType {
    fn from(n: c_int) -> Self {
        use self::MouseEventType::*;
        match n {
            0 => MouseMove,
            1 => LButtonDown,
            2 => RButtonDown,
            3 => MButtonDown,
            4 => LButtonUp,
            5 => RButtonUp,
            6 => MButtonUp,
            7 => LButtonClick,
            8 => RButtonClick,
            9 => MButtonClick,
            10 => MouseWheel,
            11 => MouseHWheel,
            _ => panic!("cv::MouseEventType: unknown C mouse event type"),
        }
    }
}

/// Provides some highgui functionallity
pub trait Show {
    /// Calls out to highgui to show the image, the duration is specified by `delay`.
    /// Returns the key that was pressed, if a key was pressed.
    fn show(&self, name: &str, delay: Option<u32>) -> Result<Option<u32>, Error>;
}

impl Show for Mat {
    fn show(&self, name: &str, delay: Option<u32>) -> Result<Option<u32>, Error> {
        let s = CString::new(name)?;
        unsafe {
            native::cvsys_nat_imshow((&s).as_ptr(), self.inner);
            let key = native::cvsys_nat_wait_key(delay.unwrap_or(0) as c_int) as i32;
            Ok(if key == -1 {
                None
            } else {
                Some(key as u32)
            })
        }
    }
}
