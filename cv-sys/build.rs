extern crate bindgen;
extern crate cmake;

use bindgen::Builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut core_modules = vec![
        "core",
        "features2d",
        "flann",
        "highgui",
        "imgcodecs",
        "imgproc",
        "objdetect",
        "videoio",
        "video",
    ];

    // Add associated CUDA files if the `cuda` feature is enabled.
    if env::var("CARGO_FEATURE_CUDA").is_ok() {
        core_modules.push("cudaobjdetect");
    }

    let contrib_modules = vec!["xfeatures2d", "img_hash", "text"];

    let all_modules = core_modules.iter().chain(contrib_modules.iter());

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Path to contrib modules.
    let contrib_modules_path = manifest_dir.join("opencv_contrib").join("modules");
    // Global configuration for OpenCV build.
    let mut config = Config::new(".");
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "RELEASE")
        .define("INSTALL_PYTHON_EXAMPLES", "OFF")
        .define("INSTALL_C_EXAMPLES", "OFF")
        .define("OPENCV_ENABLE_NONFREE", "ON")
        .define("OPENCV_EXTRA_MODULES_PATH", contrib_modules_path.into_os_string())
        .define(
            "WITH_CUDA",
            if env::var("CARGO_FEATURE_CUDA").is_ok() {
                "ON"
            } else {
                "OFF"
            },
        )
        .define("BUILD_opencv_python", "OFF")
        .define("BUILD_opencv_python2", "OFF")
        .define("BUILD_opencv_python3", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_PERF_TESTS", "OFF")
        .define("BUILD_DOCS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("CV_CORE_MODULES", core_modules.join(";"))
        .define("CV_CONTRIB_MODULES", contrib_modules.join(";"));

    // Handle OS-specific requirements.
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("linux") => {
            println!("cargo:rustc-link-lib=gomp");
            println!("cargo:rustc-link-lib=stdc++");
        }
        _ => {}
    }

    // Build OpenCV and add it to cargo.
    let dst = config.build();

    // Link cvsys.
    println!("cargo:rustc-link-search=native={}", dst.join("build").join("Debug").display());
    println!("cargo:rustc-link-lib=static=cvsys");

    let bindings = Builder::default().rustfmt_bindings(true).whitelist_function("cvsys::.*").whitelist_type("cvsys::.*")
        .opaque_type("cv::.*").opaque_type("std::.*")
        .blacklist_function("cv::.*").blacklist_function("std::.*");
    let bindings = bindings.clang_args(core_modules.iter().map(|lib| {
        format!(
            "-I{}",
            manifest_dir
                .join("opencv")
                .join("modules")
                .join(lib)
                .join("include")
                .display()
        )
    }));
    let bindings = bindings.clang_args(contrib_modules.iter().map(|lib| {
        format!(
            "-I{}",
            manifest_dir
                .join("opencv_contrib")
                .join("modules")
                .join(lib)
                .join("include")
                .display()
        )
    }));
    let bindings = bindings.clang_arg(format!("-I{}", dst.join("include").display()));
    let bindings = all_modules
        .map(|lib| format!("native/{}.hpp", lib))
        .fold(bindings, Builder::header);
    for p in bindings.command_line_flags() {
        eprintln!("{}", p);
    }
    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
