//! Holistic word
use super::private::*;
use super::*;
use errors::*;
use std::os::raw::c_char;
use std::path::Path;
use *;

extern "C" {
    fn cv_holistic_new(
        archive_file: *const c_char,
        weights_file: *const c_char,
        words_file: *const c_char,
        result: *mut CResult<*mut COCR>,
    );
    fn cv_holistic_drop(ocr: *mut COCR);
}

/// `OcrHolisticWord` class provides an interface with the tesseract-ocr API
#[derive(Debug)]
pub struct OcrHolisticWord {
    value: *mut COCR,
}

impl OcrHolisticWord {
    /// Creates an instance of the `OcrHolisticWord` class.
    pub fn new<PArch: AsRef<Path>, PWeights: AsRef<Path>, PWords: AsRef<Path>>(
        archive_file: PArch,
        weights_file: PWeights,
        words_file: PWords,
    ) -> Result<Self, Error> {
        let archive_file = path_to_cstring(archive_file)?;
        let weights_file = path_to_cstring(weights_file)?;
        let words_file = path_to_cstring(words_file)?;

        let c_archive_file = archive_file.as_ptr();
        let c_weights_file = weights_file.as_ptr();
        let c_words_file = words_file.as_ptr();

        let result = CResult::<*mut COCR>::from_callback(|r| unsafe {
            cv_holistic_new(c_archive_file, c_weights_file, c_words_file, r)
        });
        let result: Result<_, String> = result.into();
        let result = result.map_err(CvError::UnknownError)?;
        Ok(Self { value: result })
    }
}

impl Drop for OcrHolisticWord {
    fn drop(&mut self) {
        unsafe {
            cv_holistic_drop(self.value);
        }
    }
}

impl OcrImpl for OcrHolisticWord {
    fn get_value(&self) -> *mut COCR {
        self.value
    }
}

impl OcrImplInterface for OcrHolisticWord {}
