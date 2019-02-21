//! This library primarily provides a binding and API for OpenCV 3.x.
//!
//! This is a work-in-progress and modules/functions are implemented as
//! needed. Attempts to use
//! [rust-bindgen](https://github.com/servo/rust-bindgen) or
//! [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust) haven't been very
//! successful (I probably haven't tried hard enough). There is another port
//! [opencv-rust](https://github.com/kali/opencv-rust/) which generates OpenCV
//! bindings using a Python script.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate bytes;
#[macro_use]
extern crate failure;

pub mod core;
#[cfg(feature = "cuda")]
pub mod cuda;
pub mod errors;
pub mod features2d;
pub mod hash;
pub mod highgui;
pub mod imgcodecs;
pub mod imgproc;
pub mod mat;
pub mod objdetect;
#[cfg(feature = "text")]
pub mod text;
pub mod video;
pub mod videoio;

pub use core::*;
pub use mat::*;

use errors::*;
use failure::Error;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::path::Path;

extern "C" {
    fn c_drop(value: *mut c_void);
}

#[repr(C)]
struct CResult<T: Copy> {
    value: T,
    error: CDisposableString,
}

#[repr(C)]
struct CEmptyResult {
    error: CDisposableString,
}

impl<T: Copy> Into<Result<T, String>> for CResult<T> {
    fn into(self) -> Result<T, String> {
        if self.error.value.is_null() {
            Ok(self.value)
        } else {
            unsafe {
                let c_str = std::ffi::CStr::from_ptr(self.error.value);
                let err = c_str.to_string_lossy().into_owned();
                Err(err)
            }
        }
    }
}

impl<T: Copy> CResult<T> {
    pub fn from_callback<F: FnOnce(*mut CResult<T>)>(func: F) -> CResult<T> {
        let mut result: CResult<T>;
        unsafe {
            result = mem::uninitialized();
            let result_ref: *mut CResult<T> = &mut result;
            func(result_ref);
        };
        result
    }
}

impl Into<Result<(), String>> for CEmptyResult {
    fn into(self) -> Result<(), String> {
        if self.error.value.is_null() {
            Ok(())
        } else {
            unsafe {
                let c_str = std::ffi::CStr::from_ptr(self.error.value);
                let err = c_str.to_string_lossy().into_owned();
                Err(err)
            }
        }
    }
}

#[repr(C)]
struct CDisposableString {
    value: *mut c_char,
}

impl Drop for CDisposableString {
    fn drop(&mut self) {
        if !self.value.is_null() {
            unsafe { c_drop(self.value as *mut _) }
        }
    }
}

impl Default for CDisposableString {
    fn default() -> Self {
        Self {
            value: ::std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct CVecView<T: Sized> {
    array: *mut T,
    size: usize,
}

fn pack<T, U: Sized, F>(v: &Vec<T>, mut f: F) -> CVecView<U>
where
    F: FnMut(&T) -> U,
{
    let mut mapped: Vec<_> = v.iter().map(|i| f(i)).collect();
    let size = mapped.len();
    let capacity = mapped.capacity();
    let array = mapped.as_mut_ptr();
    assert_eq!(size, capacity);
    mem::forget(mapped);
    CVecView { array, size }
}

pub(crate) trait Pack {
    type In;
    fn pack(v: &Self::In) -> Self;
}

impl<T: Copy> Pack for T {
    type In = T;
    fn pack(v: &T) -> Self {
        *v
    }
}

impl<T: Pack> Pack for CVecView<T> {
    type In = Vec<T::In>;
    fn pack(v: &Self::In) -> Self {
        pack(v, |e| Pack::pack(e))
    }
}

impl<T> Drop for CVecView<T> {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.array, self.size, self.size);
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct CVec<T: Sized + NestedVec> {
    array: *mut T,
    size: usize,
}

// Unsafe because CVec is not guaranteed to contain valid pointer and size
unsafe fn unpack<T: NestedVec, U, F>(v: &CVec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    (0..v.size).map(|i| f(&*v.array.offset(i as isize))).collect()
}

pub(crate) trait Unpack {
    type Out;
    fn unpack(&self) -> Self::Out;
}

impl<T: Unpack + NestedVec> Unpack for CVec<T> {
    type Out = Vec<T::Out>;
    fn unpack(&self) -> Self::Out {
        unsafe { unpack(self, |e| e.unpack()) }
    }
}

impl<T: Copy> Unpack for T {
    type Out = T;
    fn unpack(&self) -> Self::Out {
        *self
    }
}

pub(crate) trait NestedVec {
    const LEVEL: u32;
}

impl<T: NestedVec> NestedVec for CVec<T> {
    const LEVEL: u32 = T::LEVEL + 1;
}

impl<T: Copy> NestedVec for T {
    const LEVEL: u32 = 0;
}

impl NestedVec for CDisposableString {
    const LEVEL: u32 = 0;
}

impl<T: NestedVec> Default for CVec<T> {
    fn default() -> Self {
        CVec {
            array: ::std::ptr::null_mut(),
            size: 0,
        }
    }
}

impl<T: NestedVec> Drop for CVec<T> {
    fn drop(&mut self) {
        extern "C" {
            fn cv_vec_drop(vec: *mut c_void, depth: u32);
        }
        unsafe {
            let depth = CVec::<T>::LEVEL;
            let self_ptr: *mut _ = self;
            let self_ptr: *mut c_void = self_ptr as *mut _;
            cv_vec_drop(self_ptr, depth);
        }
    }
}

impl Unpack for CDisposableString {
    type Out = String;

    fn unpack(&self) -> Self::Out {
        unsafe { CStr::from_ptr(self.value) }.to_string_lossy().into_owned()
    }
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Result<CString, Error> {
    let path = path.as_ref();
    let x = path.to_str().ok_or(CvError::InvalidPath(path.into()))?;
    let result = CString::new(x)?;
    Ok(result)
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct COption<T> {
    has_value: bool,
    value: T,
}
