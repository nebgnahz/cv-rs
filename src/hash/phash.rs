//! PHash

use super::private::*;
use super::*;

extern "C" {
    fn cv_phash_new() -> *mut CHash;
    fn cv_phash_drop(phash: *mut CHash);
}

/// Slower than average_hash, but tolerant of minor modifications
#[derive(Debug)]
pub struct PHash {
    value: *mut CHash,
}

impl PHash {
    /// Creates new PHash
    pub fn new() -> PHash {
        let value = unsafe { cv_phash_new() };
        Self { value }
    }
}

impl Drop for PHash {
    fn drop(&mut self) {
        unsafe {
            cv_phash_drop(self.value);
        }
    }
}

impl HashImpl for PHash {
    fn get_value(&self) -> *mut CHash {
        self.value
    }
}

impl HashImplInterface for PHash {}
