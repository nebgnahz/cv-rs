//! Errors for OpenCV bindings
#![allow(missing_docs)]

use std::path::PathBuf;

error_chain!{
    foreign_links {
        CStringConvert(::std::ffi::NulError);
    }

    errors {
        InvalidPath(p: PathBuf) {
            description("invalid path"),
            display("invalid path: '{:?}'", p),
        }
        NumFromPrimitive(n: i64) {
            description("fail to convert from primitive "),
            display("fail to convert from primitive: '{:?}'", n),
        }
    }
}
