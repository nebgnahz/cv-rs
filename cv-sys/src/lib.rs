#![allow(unknown_lints)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![allow(all)]

use std::ffi::CStr;
use std::iter::FromIterator;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl From<EmptyResult> for std::result::Result<(), String> {
    fn from(e: EmptyResult) -> Self {
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

impl<T> Into<std::result::Result<T, String>> for crate::Result<T> {
    fn into(self) -> std::result::Result<T, String> {
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

impl<T> Into<std::option::Option<T>> for crate::COption<T> {
    fn into(self) -> std::option::Option<T> {
        if self.hasValue {
            Some(self.value)
        } else {
            None
        }
    }
}

impl<T> Into<Vec<T>> for CVec<T>
where
    T: Clone,
{
    fn into(self) -> Vec<T> {
        Vec::from_iter((0..self.size).map(|n| unsafe { &*self.array.add(n) }).cloned())
    }
}
