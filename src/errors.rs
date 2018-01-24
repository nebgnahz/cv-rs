//! Errors for OpenCV bindings
use std::path::PathBuf;

#[derive(Debug, Fail)]
/// Custom errors that may happen during calls
pub enum CvError {
    #[fail(display = "invalid path: {:?}", path)]
    /// Indicates that path was invalid
    InvalidPath {
        /// Path that caused an error
        path: PathBuf,
    },
    #[fail(display = "failed to convert from primitive: {}", value)]
    /// Indicates that conversion from primitive to enum type is failed
    EnumFromPrimitiveConversionError {
        /// Value that caused an error
        value: i32,
    },
}