#![allow(unknown_lints)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use std::ffi::CStr;
use std::iter::FromIterator;

// Include bindings.
#[cfg(feature = "gen-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(not(feature = "gen-bindings"))]
include!("bindings.rs");

// Create opaque types.
pub enum cv_BOWKMeansTrainer {}
pub enum cv_CascadeClassifier {}
pub enum cv_HOGDescriptor {}
pub enum cv_Mat {}
pub enum cv_TermCriteria {}
pub enum cv_VideoCapture {}
pub enum cv_VideoWriter {}
pub enum cv_DMatch {}
pub enum cv_KeyPoint {}
pub enum cv_Point {}
pub enum cv_Rect {}
pub enum std_string {}

impl Clone for cvsys_Point2i {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
}

impl Clone for cvsys_Point2f {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
}

impl Clone for cvsys_Rect {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl Clone for cvsys_DMatch {
    fn clone(&self) -> Self {
        Self {
            distance: self.distance,
            imgIdx: self.imgIdx,
            queryIdx: self.queryIdx,
            trainIdx: self.trainIdx,
        }
    }
}

impl Clone for cvsys_KeyPoint {
    fn clone(&self) -> Self {
        Self {
            pt: self.pt.clone(),
            size: self.size,
            angle: self.angle,
            response: self.response,
            octave: self.octave,
            class_id: self.class_id,
        }
    }
}

impl From<cvsys_EmptyResult> for Result<(), String> {
    fn from(e: cvsys_EmptyResult) -> Self {
        if unsafe { e.error.is_str() } {
            Err(unsafe { CStr::from_ptr(e.error.get_str()) }
                .to_str()
                .expect("got non-UTF8 error string from OpenCV")
                .to_owned())
        } else {
            Ok(())
        }
    }
}

impl<T> Into<std::result::Result<T, String>> for cvsys_Result<T> {
    fn into(self) -> Result<T, String> {
        if unsafe { self.error.is_str() } {
            Err(unsafe { CStr::from_ptr(self.error.get_str()) }
                .to_str()
                .expect("got non-UTF8 error string from OpenCV")
                .to_owned())
        } else {
            Ok(self.value)
        }
    }
}

impl<T> Into<Option<T>> for cvsys_COption<T> {
    fn into(self) -> Option<T> {
        if self.hasValue {
            Some(self.value)
        } else {
            None
        }
    }
}

impl<T, U> Into<Vec<U>> for cvsys_CVec<T>
where
    T: Into<U> + Clone,
{
    fn into(self) -> Vec<U> {
        Vec::from_iter(
            (0..self.size)
                .map(|n| unsafe { &*self.array.add(n) })
                .cloned()
                .map(Into::into),
        )
    }
}

impl<T> cvsys_CVec<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        unsafe { std::slice::from_raw_parts(self.array, self.size).iter() }
    }
}
