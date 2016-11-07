extern crate getopts;
extern crate rust_vision;

use rust_vision::*;
use rust_vision::cuda::*;
use rust_vision::objdetect::SvmDetector;
use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let args: Vec<String> = ::std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optopt("d", "", "set the directory to look for images", "DIRECTORY");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(&program, opts);
            ::std::process::exit(-1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    let dir = matches.opt_str("d").expect("You need to provide the directory");
    let path = Path::new(&dir);
    for entry in try!(fs::read_dir(path)) {
        let dir = try!(entry);
        println!("{:?}", dir.path());

        let mut hog = GpuHog::default();
        let detector = SvmDetector::default_people_detector();
        hog.set_svm_detector(detector);
        run_hog_for_path(&mut hog, dir.path());
    }
    Ok(())
}

fn run_hog_for_path<P: AsRef<Path>>(hog: &mut GpuHog, path: P) {
    let mut buf = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);
    let mut gpu_mat = GpuMat::default();
    gpu_mat.upload(&mat);
    let start = ::std::time::Instant::now();
    let results = hog.detect(&gpu_mat);
    let elapsed = start.elapsed();
    println!("{} ms",
             elapsed.as_secs() as f64 * 1_000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0);

    highgui_named_window("window", WindowFlags::WindowAutosize);
    results.iter()
        .map(|&r| mat.rectangle(r.scale(0.6)))
        .count();
    mat.show("window", 0);
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] DIRECTORY", program);
    print!("{}", opts.usage(&brief));
}
