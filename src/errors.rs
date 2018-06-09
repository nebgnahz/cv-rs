//! Errors for OpenCV bindings
use std::path::PathBuf;

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
