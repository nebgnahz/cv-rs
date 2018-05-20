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
use mat::CMat;
use *;

extern "C" {
    fn cv_ocr_run(
        ocr: *const private::COCR,
        image: *const CMat,
        output_text: *mut CDisposableString,
        component_rects: *mut CVec<Rect>,
        component_texts: *mut CVec<CDisposableString>,
        component_confidences: *mut CVec<f32>,
        component_level: ComponentLevel,
    );
}

mod private {
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub enum COCR {}

    pub trait OcrImpl {
        fn get_value(&self) -> *mut COCR;
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
        let mut output_text = CDisposableString::default();
        let mut component_rects = CVec::<Rect>::default();
        let mut component_texts = CVec::<CDisposableString>::default();
        let mut component_confidences = CVec::<f32>::default();
        unsafe {
            cv_ocr_run(
                value,
                image.inner,
                &mut output_text,
                &mut component_rects,
                &mut component_texts,
                &mut component_confidences,
                component_level,
            );
        }
        (
            output_text.unpack(),
            component_rects.unpack(),
            component_texts.unpack(),
            component_confidences.unpack(),
        )
    }
}
