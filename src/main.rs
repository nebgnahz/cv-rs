extern crate getopts;
extern crate rust_vision;

use rust_vision::*;
// use rust_vision::cuda::GpuHog as Hog;
use rust_vision::objdetect::*;
use rust_vision::objdetect::HogDescriptor as Hog;
use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

fn main() {
    run().unwrap();
}

fn duration_as_ms(d: ::std::time::Duration) -> f64 {
    d.as_secs() as f64 * 1_000.0 + d.subsec_nanos() as f64 / 1_000_000.0
}

fn run() -> Result<()> {
    let args: Vec<String> = ::std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optopt("d", "dir", "the directory to look for images", "DIRECTORY");
    opts.optflag("m", "mot", "using MOT dataset");
    opts.optflag("s", "show", "display the detection results");
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

    let show = matches.opt_present("s");
    let mot = matches.opt_present("m");

    let mut dir = matches.opt_str("d").expect("You need to provide the directory");

    if show {
        highgui_named_window("window", WindowFlags::WindowAutosize);
    }

    let mut params = HogParams::default();
    params.scale = 1.05;
    params.padding = Size2i::new(16, 16);
    let mut hog = Hog::with_params(params);
    println!("{:?}", hog.params);

    let detector = SvmDetector::default_people_detector();
    hog.set_svm_detector(detector);

    if mot {
        dir.push_str("/img1");
        run_for_mot(&mut hog, Path::new(&dir), show)
    } else {
        for entry in try!(fs::read_dir(Path::new(&dir))) {
            let dir = try!(entry);
            // println!("Processing {:?}", dir.path());
            run_detect_for_image(&mut hog, dir.path(), show);
        }
        Ok(())
    }
}

fn run_detect_for_image<P: AsRef<Path>, OD: ObjectDetect>(detector: &mut OD, path: P, show: bool) {
    let mut buf = Vec::new();
    let filename = path.as_ref().file_stem().unwrap().to_string_lossy().into_owned();
    let frame_num = filename; // .parse::<usize>().unwrap();
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

    let start = ::std::time::Instant::now();
    let results = detector.detect(&mat);
    let elapsed = start.elapsed();

    println!("{},{},{}",
             frame_num,
             results.len(),
             duration_as_ms(elapsed));

    if show {
        results.iter()
            .map(|&(r, _w)| mat.rectangle(r))
            .count();
        mat.show("window", 0);
    }
}

/// Process MOT dataset.
fn run_for_mot<P: AsRef<Path>, OD: ObjectDetect>(detector: &mut OD, folder: P, show: bool) -> Result<()> {
    let img_folder = folder.as_ref();
    let mut entries = try!(fs::read_dir(img_folder))
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    println!("frame,count,time");
    entries.sort_by_key(|a| a.file_name());
    entries.iter()
        .map(|entry| {
            // filename contains frame number (such as 00001.jpg)
            // If failed, we continue
            if let Ok(frame_num) = entry.path().file_stem().unwrap().to_str().unwrap().parse::<usize>() {
                // Read Image file
                let mut buf = Vec::new();
                File::open(entry.path()).unwrap().read_to_end(&mut buf).unwrap();
                let mat = Mat::imdecode(&buf, ImreadModes::ImreadGrayscale);

                let start = ::std::time::Instant::now();
                let results = detector.detect(&mat);
                let elapsed = start.elapsed();

                // smoothed = (0.3 * smoothed as f64 + 0.7 * (results.len() as f64 - smoothed as f64)) as usize;
                println!("{},{},{}",
                         frame_num,
                         results.len(),
                         duration_as_ms(elapsed));

                if show {
                    results.iter()
                        .map(|&(r, _w)| mat.rectangle(r.scale(0.6)))
                        .count();
                    mat.show("window", 0);
                }
            }
        })
        .collect::<Vec<_>>();
    Ok(())
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] DIRECTORY", program);
    print!("{}", opts.usage(&brief));
}
