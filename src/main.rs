extern crate rust_vision;

use rust_vision::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/lenna.png");

    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let flags = Vec::new();
    if let Some(encoded) = mat.imencode(".JPEG", flags) {
        println!("{}", encoded.len());

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets/lenna.jpg");
        File::create(d).unwrap().write_all(&encoded).unwrap();
    }
}
