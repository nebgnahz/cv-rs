//! Tesseract
use ::*;
use core::*;
use std::ffi::CString;
use std::path::Path;

mod private {
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub enum COCR {}

    pub trait OcrImpl {
        fn get_value(&self) -> *mut COCR;
    }
}


extern "C" {
    fn cv_tesseract_new(
        data_path: *const c_char,
        language: *const c_char,
        char_whitelist: *const c_char,
        oem: EngineMode,
        psmode: PageSegmentationMode,
    ) -> *mut private::COCR;
    fn cv_tesseract_drop(ocr: *mut private::COCR);
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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum ComponentLevel {
    Word,
    TextLine,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum EngineMode {
    TesseractOnly,
    CubeOnly,
    TesseractCubeCombined,
    Default,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum PageSegmentationMode {
    OsdOnly,
    AutoOsd,
    AutoOnly,
    Auto,
    SingleColumn,
    SingleBlockVertText,
    SingleBlock,
    SingleLine,
    SingleWord,
    CircleWord,
    SingleChar,
}

#[allow(missing_docs)]
pub trait OcrImplInterface: private::OcrImpl {}

/// Basic trait for all OCR types
pub trait Ocr {
    /// Recognize text
    fn run(&self, image: &Mat, component_level: ComponentLevel) -> (String, Vec<Rect>, Vec<String>, Vec<f32>);
}

/// `OcrTesseract` class provides an interface with the tesseract-ocr API
#[derive(Debug)]
pub struct OcrTesseract {
    value: *mut private::COCR,
}

impl OcrTesseract {
    /// Creates an instance of the `OcrTesseract` class. Initializes Tesseract.
    pub fn new(
        data_path: Option<&Path>,
        language: Option<&str>,
        char_whitelist: Option<&Vec<c_char>>,
        oem: EngineMode,
        psmode: PageSegmentationMode,
    ) -> Self {
        let value = unsafe {
            let c_data_path = data_path.map(|x| CString::new(x.to_str().unwrap()).unwrap());
            let c_language = language.map(|x| CString::new(x).unwrap());
            let c_char_whitelist = char_whitelist.map(|x| x.as_ptr());

            let c_data_path = to_nullable_string(c_data_path);
            let c_language = to_nullable_string(c_language);
            let c_char_whitelist = unwrap_or_null(c_char_whitelist);


            cv_tesseract_new(c_data_path, c_language, c_char_whitelist, oem, psmode)
        };
        Self { value }
    }
}

impl Drop for OcrTesseract {
    fn drop(&mut self) {
        unsafe {
            cv_tesseract_drop(self.value);
        }
    }
}

impl private::OcrImpl for OcrTesseract {
    fn get_value(&self) -> *mut private::COCR {
        self.value
    }
}

impl OcrImplInterface for OcrTesseract {}

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

unsafe fn to_nullable_string(value: Option<CString>) -> *const c_char {
    unwrap_or_null(value.map(|x| x.as_ptr()))
}

unsafe fn unwrap_or_null(value: Option<*const c_char>) -> *const c_char {
    value.unwrap_or(::std::ptr::null())
}
