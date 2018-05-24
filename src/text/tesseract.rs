//! Tesseract
use super::private::*;
use super::*;
use errors::*;
use std::os::raw::c_char;
use std::path::Path;
use *;

extern "C" {
    fn cv_tesseract_new(
        data_path: *const c_char,
        language: *const c_char,
        char_whitelist: *const c_char,
        oem: EngineMode,
        psmode: PageSegmentationMode,
        result: *mut CResult<*mut COCR>,
    );
    fn cv_tesseract_drop(ocr: *mut COCR);
}

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
        let c_data_path = path_to_cstring!(data_path);
        let c_language = string_to_cstring!(language);
        let c_char_whitelist = string_to_cstring!(char_whitelist);

        let c_data_path = to_nullable_string(&c_data_path);
        let c_language = to_nullable_string(&c_language);
        let c_char_whitelist = to_nullable_string(&c_char_whitelist);

        let result = CResult::<*mut COCR>::from_callback(|r| unsafe {
            cv_tesseract_new(c_data_path, c_language, c_char_whitelist, oem, psmode, r)
        });
        let result: Result<_, String> = result.into();
        let result = result.map_err(CvError::UnknownError)?;
        Ok(Self { value: result })
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

fn to_nullable_string(value: &Option<CString>) -> *const c_char {
    unwrap_or_null(&value.as_ref().map(|x| x.as_ptr()))
}

fn unwrap_or_null(value: &Option<*const c_char>) -> *const c_char {
    value.unwrap_or(::std::ptr::null())
}
