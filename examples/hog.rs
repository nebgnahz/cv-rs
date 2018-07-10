extern crate cv;
extern crate getopts;

use cv::highgui::*;
use cv::imgcodecs::*;
use cv::objdetect::*;
use cv::*;

#[cfg(feature = "cuda")]
use cv::cuda::GpuHog as Hog;

#[cfg(not(feature = "cuda"))]
use cv::objdetect::HogDescriptor as Hog;

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
    opts.optopt("d", "dir", "the directory to look for images", "DIRECTORY");
    opts.optflag("m", "measure", "measure the execution time (report in ms)");
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
    let measure = matches.opt_present("m");

    let dir = matches.opt_str("d").expect("You need to provide the directory");

    if show {
        highgui_named_window("window", WindowFlag::Autosize).unwrap();
    }

    let mut param = HogParams::default();
    param.group_threshold = 0;
    let mut hog = Hog::with_params(param);
    let detector = SvmDetector::default_people_detector();
    hog.set_svm_detector(detector);

    for entry in fs::read_dir(Path::new(&dir))? {
        let dir = entry?;
        println!("Processing {:?}", dir.path());
        run_detect_for_image(&mut hog, dir.path(), show, measure);
    }
    Ok(())
}

fn run_detect_for_image<P: AsRef<Path>, OD: ObjectDetect>(detector: &mut OD, path: P, show: bool, measure: bool) {
    let mut buf = Vec::new();
    let filename = path.as_ref().file_stem().unwrap().to_string_lossy().into_owned();
    let frame_num = filename.parse::<usize>().unwrap();
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let mat = Mat::image_decode(&buf, ImageReadMode::Grayscale);

    let start = ::std::time::Instant::now();
    let results = detector.detect(&mat);
    let elapsed = start.elapsed();

    print!("{},{},", frame_num, results.len());
    if measure {
        println!(
            "{}",
            elapsed.as_secs() as f64 * 1_000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0
        );
    }

    if show {
        results.iter().map(|&(r, _w)| mat.rectangle(r.scale(0.6))).count();
        mat.show("window", 0).unwrap();
    }
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] DIRECTORY", program);
    print!("{}", opts.usage(&brief));
}
