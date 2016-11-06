extern crate getopts;
extern crate rust_vision;

use rust_vision::*;
use rust_vision::objdetect::*;
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
        run_hog_for_path(dir.path());
    }
    Ok(())
}

fn run_hog_for_path<P: AsRef<Path>>(path: P) {
    let mut buf = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let mut hog = HogDescriptor::new();
    let detector = SvmDetector::default_people_detector();
    hog.set_svm_detector(detector);

    let start = ::std::time::Instant::now();
    let results = hog.detect(&mat, Size2i::new(2, 2), Size2i::new(4, 4), 1.1);

    let elapsed = start.elapsed();
    println!("{} ms",
             elapsed.as_secs() as f64 * 1_000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0);
    println!("{:?}", results);

    highgui_named_window("window", WindowFlags::WindowAutosize);

    // we draw each of them on the image
    results.iter()
        .map(|&(r, _w)| mat.rectangle(r.scale(0.6)))
        .count();
    mat.show("window", 0);
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] DIRECTORY", program);
    print!("{}", opts.usage(&brief));
}

// Read the ground truth data, csv file
fn mot_ground_truth<P: AsRef<Path>>(path: P) -> Result<()> {
    let result = HashMap::new();

    let f = try!(File::open(path));
    let f = BufReader::new(f);
    for line in f.lines() {
        println!("{}", line.unwrap());

        // 1,1,1363,569,103,241,1,1,0.86014
        // <frame>, <id>, <left>, <top>, <width>, <height>, <?>, <?>, conf
        let v = line.split(',');
        let id = try!(v[1].parse::<int>());
        let frame = try!(v[0].parse::<int>());

        player_stats.entry(id).or_insert(1);
        println!();
    }
}
