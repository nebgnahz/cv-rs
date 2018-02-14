//! Tesseract
use super::*;
use super::private::*;
use ::*;
use errors::*;
use std::os::raw::c_char;
use std::path::Path;

extern "C" {
    fn cv_tesseract_new(
        data_path: *const c_char,
        language: *const c_char,
        char_whitelist: *const c_char,
        oem: EngineMode,
        psmode: PageSegmentationMode,
    ) -> *mut COCR;
    fn cv_tesseract_drop(ocr: *mut COCR);
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

/// `OcrTesseract` class provides an interface with the tesseract-ocr API
#[derive(Debug)]
pub struct OcrTesseract {
    value: *mut COCR,
}

impl OcrTesseract {
    /// Creates an instance of the `OcrTesseract` class. Initializes Tesseract.
    pub fn new(
        data_path: Option<&Path>,
        language: Option<&str>,
        char_whitelist: Option<&str>,
        oem: EngineMode,
        psmode: PageSegmentationMode,
    ) -> Result<Self, Error> {
        let value = unsafe {
            let c_data_path = match data_path {
                Some(x) => {
                    let x = x.to_str().ok_or(CvError::InvalidPath(x.into()))?;
                    Some(CString::new(x)?)
                }
                None => None,
            };
            let c_language = match language {
                Some(x) => Some(CString::new(x)?),
                None => None,
            };
            let c_char_whitelist = match char_whitelist {
                Some(x) => Some(CString::new(x)?),
                None => None,
            };

            let c_data_path = to_nullable_string(&c_data_path);
            let c_language = to_nullable_string(&c_language);
            let c_char_whitelist = to_nullable_string(&c_char_whitelist);

            cv_tesseract_new(c_data_path, c_language, c_char_whitelist, oem, psmode)
        };
        Ok(Self { value })
    }
}

impl Drop for OcrTesseract {
    fn drop(&mut self) {
        unsafe {
            cv_tesseract_drop(self.value);
        }
    }
}

impl OcrImpl for OcrTesseract {
    fn get_value(&self) -> *mut COCR {
        self.value
    }
}

impl OcrImplInterface for OcrTesseract {}
