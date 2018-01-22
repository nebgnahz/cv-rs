//! Errors for OpenCV bindings
#![allow(missing_docs)]

use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum CvError {
    #[fail(display = "Value was null")]
    NullError,
    #[fail(display = "invalid path: {:?}", path)]
    InvalidPath{
        path: PathBuf,
    },
    #[fail(display = "failed to convert from primitive: {}", value)]
    EnumFromPrimitiveConversionError {
        value: i32,
    }
}
