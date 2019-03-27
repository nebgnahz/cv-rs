//! Tesseract
use super::private::*;
use super::*;
use errors::*;
use std::os::raw::c_char;
use std::path::Path;
use *;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum EngineMode {
    TesseractOnly,
    CubeOnly,
    TesseractCubeCombined,
    Default,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
    value: *mut native::cvsys_OCRTesseract,
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
        let c_data_path = path_to_cstring!(data_path);
        let c_language = string_to_cstring!(language);
        let c_char_whitelist = string_to_cstring!(char_whitelist);

        let c_data_path = to_nullable_string(&c_data_path);
        let c_language = to_nullable_string(&c_language);
        let c_char_whitelist = to_nullable_string(&c_char_whitelist);

        let result = unsafe {
            native::cvsys_tesseract_new(c_data_path, c_language, c_char_whitelist, oem as i32, psmode as i32)
        };
        let result: Result<_, String> = result.into();
        let result = result.map_err(CvError::UnknownError)?;
        Ok(Self { value: result })
    }
}

impl Drop for OcrTesseract {
    fn drop(&mut self) {
        unsafe {
            native::cvsys_tesseract_drop(self.value);
        }
    }
}

impl OcrImpl for OcrTesseract {
    fn get_value(&self) -> *mut native::cvsys_BaseOCR {
        self.value as *mut _
    }
}

impl OcrImplInterface for OcrTesseract {}

fn to_nullable_string(value: &Option<CString>) -> *const c_char {
    unwrap_or_null(&value.as_ref().map(|x| x.as_ptr()))
}

fn unwrap_or_null(value: &Option<*const c_char>) -> *const c_char {
    value.unwrap_or(::std::ptr::null())
}
