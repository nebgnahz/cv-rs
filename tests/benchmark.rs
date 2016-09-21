extern crate rust_vision;
use rust_vision::ImreadModes;
use rust_vision::Mat;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

fn timed<F>(label: &str, mut inner: F)
    where F: FnMut()
{
    let start = Instant::now();
    inner();
    let elapsed = start.elapsed();
    println!("  {}: {} ms",
             label,
             elapsed.as_secs() as f64 * 1_000.0 +
             elapsed.subsec_nanos() as f64 / 1_000_000.0);
}

#[test]
fn bench_mat_new() {
    timed("decode lenna.png", || {
        Mat::new();
    });
}

#[test]
fn bench_decode_lenna() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets/lenna.png");
    let mut buf = Vec::new();
    File::open(d).unwrap().read_to_end(&mut buf).unwrap();

    timed("decode lenna.png", || {
        Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);
    });
}
