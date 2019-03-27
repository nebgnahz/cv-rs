//! Provides different algorithms for text detection and recognition in natural scene images
#[macro_use]
mod macros;
mod hmm;
mod holisticword;
#[cfg(feature = "tesseract")]
mod tesseract;
pub use self::hmm::*;
pub use self::holisticword::*;
#[cfg(feature = "tesseract")]
pub use self::tesseract::*;

use failure::Error;
use std::ffi::CStr;
use *;

mod private {
    pub trait OcrImpl {
        fn get_value(&self) -> *mut native::cvsys_BaseOCR;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum ComponentLevel {
    Word,
    TextLine,
}

#[allow(missing_docs)]
pub trait OcrImplInterface: private::OcrImpl {}

/// Basic trait for all OCR types
pub trait Ocr {
    /// Recognize text
    fn run(&self, image: &Mat, component_level: ComponentLevel) -> (String, Vec<Rect>, Vec<String>, Vec<f32>);
}

impl<T: OcrImplInterface> Ocr for T {
    fn run(&self, image: &Mat, component_level: ComponentLevel) -> (String, Vec<Rect>, Vec<String>, Vec<f32>) {
        let value = self.get_value();
        unsafe {
            let mut output_text: native::cvsys_CString = std::mem::zeroed();
            let mut component_rects: native::cvsys_CVec<native::cvsys_Rect> = std::mem::zeroed();
            let mut component_texts: native::cvsys_CVec<native::cvsys_CString> = std::mem::zeroed();
            let mut component_confidences: native::cvsys_CVec<f32> = std::mem::zeroed();
            native::cvsys_ocr_run(
                value,
                image.inner,
                &mut output_text,
                &mut component_rects,
                &mut component_texts,
                &mut component_confidences,
                component_level as i32,
            );
            let component_texts = component_texts.iter().map(|s| CStr::from_ptr(s.get_str()).to_str().expect("OpenCV text gave back non-utf8 string").to_owned()).collect();
            (
                CStr::from_ptr(output_text.get_str()).to_str().expect("OpenCV text gave back non-utf8 string").to_owned(),
                component_rects.into(),
                component_texts,
                component_confidences.into(),
            )
        }
    }
}
