#![feature(test)]
extern crate test;
extern crate rust_vision;
use self::test::Bencher;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use rust_vision::Mat;
use rust_vision::ImreadModes;

#[bench]
fn bench_decode_lenna(b: &mut Bencher) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/lenna.png");
    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();

    b.iter(|| Mat::imdecode(&buf, ImreadModes::ImreadGrayscale));
}
