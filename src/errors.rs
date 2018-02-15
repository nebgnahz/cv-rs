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
    #[fail(display = "failed to convert from primitive: {}", value)]
    /// Indicates that conversion from primitive to enum type is failed
    EnumFromPrimitiveConversionError {
        /// Value that caused an error
        value: i32,
    },
    #[fail(display = "Unknown error: {:?}", _0)]
    /// Indicates that error occurred in C++ code
    UnknownError(String),
}
