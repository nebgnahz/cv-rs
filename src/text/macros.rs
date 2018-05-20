#[macro_export]
macro_rules! path_to_cstring {
    ($x:expr) => {
        match $x {
            Some(x) => {
                let x = x.to_str().ok_or(::errors::CvError::InvalidPath(x.into()))?;
                Some(CString::new(x)?)
            }
            None => None,
        };
    };
}

#[macro_export]
macro_rules! string_to_cstring {
    ($x:expr) => {
        match $x {
            Some(x) => Some(CString::new(x)?),
            None => None,
        };
    };
}
