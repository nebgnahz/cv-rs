extern crate bindgen;
extern crate cmake;

use bindgen::Builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::ffi::OsString;
use itertools::Itertools;

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
        .static_crt(true)
        .define("CMAKE_C_FLAGS", "/MT")
        .define("CMAKE_C_FLAGS_DEBUG", "/MT")
        .define("CMAKE_C_FLAGS_RELEASE", "/MT")
        .define("CMAKE_C_FLAGS_MINSIZEREL", "/MT")
        .define("CMAKE_C_FLAGS_RELWITHDEBINFO", "/MT")
        .define("CMAKE_CXX_FLAGS", "/MT")
        .define("CMAKE_CXX_FLAGS_DEBUG", "/MT")
        .define("CMAKE_CXX_FLAGS_RELEASE", "/MT")
        .define("CMAKE_CXX_FLAGS_MINSIZEREL", "/MT")
        .define("CMAKE_CXX_FLAGS_RELWITHDEBINFO", "/MT")
        .define("BUILD_WITH_STATIC_CRT", "ON")
        // This creates some really annoying build issues on Windows.
        // Eventually we need to fix it.
        .define("WITH_IPP", "OFF")
        .define("BUILD_IPP_IW", "OFF")
        .define("BUILD_opencv_apps", "OFF")
        .define("BUILD_opencv_java_bindings_generator", "OFF")
        .define("BUILD_opencv_js", "OFF")
        .define("BUILD_opencv_python_bindings_generator", "OFF")
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
        Ok("windows") => {
            println!("cargo:rustc-link-lib=comdlg32");
            println!("cargo:rustc-link-lib=Vfw32");
            println!("cargo:rustc-link-lib=Ole32");
            println!("cargo:rustc-link-lib=OleAut32");
        }
        _ => {}
    }

    // Build OpenCV and add it to cargo.
    let dst = config.build();

    // Load link_libs_list.txt to get list of all static libs to link to.
    // This is generated in the CMakeLists.txt.
    let libs_list: Vec<PathBuf> = std::fs::read_to_string(dst.join("build").join("link_libs_list.txt"))
        .expect("CMakeLists.txt didn't produce libs list")
        .replace("$(Configuration)", "Debug")
        .split(';').map(Into::into).collect();
    
    // This contains the raw lib names.
    let deduped_lib_names: Vec<OsString> =
        libs_list.iter()
            .map(|p| p.file_stem().expect("expected only files in libs list").to_os_string())
            .unique().collect();

    // This contains the lib search paths.
    let deduped_lib_search_paths: Vec<PathBuf> =
        libs_list.iter()
            .map(|p| p.parent().expect("expected libs to be in a directory").to_path_buf())
            .unique().collect();

    // Link cvsys.
    println!(
        "cargo:rustc-link-search={}",
        dst.join("build").join("Debug").display()
    );
    println!("cargo:rustc-link-lib=static=cvsys");

    // Link all cvsys dependencies.
    for libpath in deduped_lib_search_paths {
        println!(
            "cargo:rustc-link-search={}",
            libpath.display()
        );
    }

    for libname in deduped_lib_names {
        println!("cargo:rustc-link-lib=static={}d", libname.to_str().expect("OpenCV lib names must be valid UTF-8"));
    }

    let bindings = Builder::default()
        .rustfmt_bindings(true)
        .whitelist_recursively(false)
        .derive_eq(true)
        .derive_ord(true)
        .derive_hash(true)
        .derive_debug(true)
        .derive_copy(true)
        .whitelist_function("cvsys::.*")
        .whitelist_type("cvsys::.*")
        .opaque_type("cv::.*")
        .opaque_type("std::.*")
        .whitelist_type("cv::MouseCallback")
        .whitelist_type("cv::text::classifier_type");
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
    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
