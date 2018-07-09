//! Errors for OpenCV bindings
use std::ffi::CStr;
use std::path::PathBuf;
use std::os::raw::{c_int, c_char};

#[derive(Debug, Fail)]
/// Custom errors that may happen during calls
pub enum CvError {
    #[fail(display = "invalid path: {:?}", _0)]
    /// Indicates that path was invalid
    InvalidPath(PathBuf),
    #[fail(display = "EntryNotFound: {:?}", _0)]
    /// Indicates that there is no entry on specified path
    EntryNotFound(PathBuf),
    #[fail(display = "Unknown error: {:?}", _0)]
    /// Indicates that error occurred in C++ code
    UnknownError(String),
    #[fail(display = "Non ascii characters found in string: {:?}", _0)]
    /// Indicates that string contains non ascii characters
    UnicodeChars(String),
}

extern "C" {
    fn cvRedirectError(callback: unsafe extern fn (status: c_int,
                                                   func: *const c_char,
                                                   msg: *const c_char,
                                                   file_name: *const c_char,
                                                   line: c_int) -> c_int);
}

// handle that can be set by user
static mut GLOBAL_CATCH: Option<fn (status: i32) -> i32> = None;

unsafe extern "C" fn err_handler(status: c_int,
                          func_name: *const c_char,
                          err: *const c_char,
                          file_name: *const c_char,
                          line: c_int) -> c_int {
    let to_str = |p: *const c_char| -> &str { CStr::from_ptr(p).to_str().unwrap_or("") };

    println!("opencv err: {} - {} in {}:{} (status: {})",
             to_str(err),
             to_str(func_name),
             to_str(file_name),
             line,
             status);

    if let Some(catch) = GLOBAL_CATCH {
        catch(status)
    } else {
        panic!("failed in opencv function");
    }
}

/// tells opencv to print out errors
pub unsafe fn catch_opencv_errors(cb: Option<fn (status: i32) -> i32>) {
    cvRedirectError(err_handler);
    GLOBAL_CATCH = cb;
}
