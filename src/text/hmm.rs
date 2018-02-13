//! HMM
use super::*;
use super::private::*;
use ::*;
use core::CMat;
use errors::*;
use std::os::raw::c_char;
use std::path::Path;

extern "C" {
    fn cv_hmm_new(
        classifier_filename: *const c_char,
        vocabulary: *const c_char,
        transition_probabilities_table: *mut CMat,
        emission_probabilities_table: *mut CMat,
        classifier_type: ClassifierType,
    ) -> *mut COCR;
    fn cv_hmm_drop(ocr: *mut COCR);
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum ClassifierType {
    Knn,
    Cnn,
}

/// `OcrHmmDecoder` class provides an interface with the HmmDecoder-ocr API
#[derive(Debug)]
pub struct OcrHmmDecoder {
    value: *mut COCR,
}

impl OcrHmmDecoder {
    /// Creates an instance of the `OcrHmmDecoder` class. Initializes HmmDecoder.
    pub fn new(
        classifier_filename: &Path,
        vocabulary: &str,
        transition_probabilities_table: &Mat,
        emission_probabilities_table: &Mat,
        classifier_type: ClassifierType,
    ) -> Result<Self, Error> {
        let value = unsafe {
            let classifier_filename = classifier_filename.to_str().ok_or(CvError::InvalidPath {
                path: classifier_filename.into(),
            })?;
            let classifier_filename = CString::new(classifier_filename)?;
            let vocabulary = CString::new(vocabulary)?;

            cv_hmm_new(
                classifier_filename.as_ptr(),
                vocabulary.as_ptr(),
                transition_probabilities_table.inner,
                emission_probabilities_table.inner,
                classifier_type,
            )
        };
        Ok(Self { value })
    }
}

impl Drop for OcrHmmDecoder {
    fn drop(&mut self) {
        unsafe {
            cv_hmm_drop(self.value);
        }
    }
}

impl OcrImpl for OcrHmmDecoder {
    fn get_value(&self) -> *mut COCR {
        self.value
    }
}

impl OcrImplInterface for OcrHmmDecoder {}
