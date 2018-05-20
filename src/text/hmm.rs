//! HMM
use super::private::*;
use super::*;
use errors::*;
use mat::CMat;
use std::os::raw::c_char;
use std::path::Path;
use *;

extern "C" {
    fn cv_hmm_new(
        classifier_filename: *const c_char,
        vocabulary: *const c_char,
        transition_probabilities_table: *mut CMat,
        emission_probabilities_table: *mut CMat,
        classifier_type: ClassifierType,
        result: *mut CResult<*mut COCR>,
    );
    fn cv_hmm_drop(ocr: *mut COCR);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
    pub fn new<P: AsRef<Path>>(
        classifier_filename: P,
        vocabulary: &str,
        transition_probabilities_table: &Mat,
        emission_probabilities_table: &Mat,
        classifier_type: ClassifierType,
    ) -> Result<Self, Error> {
        let classifier_filename = classifier_filename.as_ref();
        if !classifier_filename.exists() {
            return Err(CvError::EntryNotFound(classifier_filename.into()).into());
        }
        let classifier_filename = classifier_filename
            .to_str()
            .ok_or(CvError::InvalidPath(classifier_filename.into()))?;
        let classifier_filename = CString::new(classifier_filename)?;
        let vocabulary = CString::new(vocabulary)?;

        let result = CResult::<*mut COCR>::from_callback(|r| unsafe {
            cv_hmm_new(
                classifier_filename.as_ptr(),
                vocabulary.as_ptr(),
                transition_probabilities_table.inner,
                emission_probabilities_table.inner,
                classifier_type,
                r,
            )
        });
        let result: Result<_, String> = result.into();
        let result = result.map_err(CvError::UnknownError)?;
        Ok(Self { value: result })
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
