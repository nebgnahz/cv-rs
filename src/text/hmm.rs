//! HMM
use super::private::*;
use super::*;
use errors::*;
use std::os::raw::c_char;
use std::path::Path;
use *;

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
    value: *mut native::cvsys_OCRHMMDecoder,
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

        let result = unsafe {
            native::cvsys_hmm_new(
                classifier_filename.as_ptr(),
                vocabulary.as_ptr(),
                transition_probabilities_table.inner,
                emission_probabilities_table.inner,
                classifier_type as u32,
            )
        };
        let result: Result<_, String> = result.into();
        let result = result.map_err(CvError::UnknownError)?;
        Ok(Self { value: result })
    }
}

impl Drop for OcrHmmDecoder {
    fn drop(&mut self) {
        unsafe {
            native::cvsys_hmm_drop(self.value);
        }
    }
}

impl OcrImpl for OcrHmmDecoder {
    fn get_value(&self) -> *mut native::cvsys_BaseOCR {
        self.value as *mut _
    }
}

impl OcrImplInterface for OcrHmmDecoder {}
