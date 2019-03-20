extern crate bindgen;
extern crate cmake;

use bindgen::Builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn link_package(name: &str) {
    let package = pkg_config::probe_library(name).expect(&format!("must install {}", name));
    for libpath in &package.link_paths {
        println!("cargo:rustc-link-search={}", libpath.display());
    }
    for lib in &package.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}

fn cmake_bool(flag: bool) -> &'static str {
    if flag {
        "ON"
    } else {
        "OFF"
    }
}

fn cmake_common(config: &mut Config, target_os: &str) {
    // Make everything static.
    config.define("BUILD_SHARED_LIBS", "OFF").profile("Release");

    // Handle OS-specific requirements.
    match target_os {
        "linux" => {
            // There may be issues with precompiled headers and binding generation.
            config.define("ENABLE_PRECOMPILED_HEADERS", "OFF");
        }
        "windows" => {
            // Really make sure we use static runtime.
            config
                .static_crt(true)
                .define("CMAKE_C_FLAGS", "/MT")
                .define("CMAKE_C_FLAGS_RELEASE", "/MT")
                .define("CMAKE_CXX_FLAGS", "/MT")
                .define("CMAKE_CXX_FLAGS_RELEASE", "/MT")
                .define("BUILD_WITH_STATIC_CRT", "ON");
        }
        _ => {}
    }
}

fn main() -> Result<(), std::io::Error> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let feature_cuda = env::var("CARGO_FEATURE_CUDA").is_ok();
    let feature_system = env::var("CARGO_FEATURE_SYSTEM").is_ok();
    let feature_gen_bindings = env::var("CARGO_FEATURE_GEN_BINDINGS").is_ok();

    let mut used_core_modules = vec![
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

    // Add associated CUDA modules if the `cuda` feature is enabled.
    if feature_cuda {
        used_core_modules.push("cudaobjdetect");
    }

    // All the contrib modules used (core and contrib).
    let used_contrib_modules = vec!["xfeatures2d", "img_hash", "text"];

    // Collect all the modules used.
    let all_used_modules: Vec<_> = used_core_modules.iter().chain(used_contrib_modules.iter()).collect();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Path to contrib modules.
    let contrib_modules_path = manifest_dir.join("opencv_contrib").join("modules");

    let (opencv_include_dir, opencv_lib_dir) = if feature_system {
        let opencv_dir = env::var("OPENCV_DIR")
            .unwrap_or_else(|_| panic!("OPENCV_DIR not set (set it to the directory the include folder is in)"));
        let opencv_lib_dir = env::var("OPENCV_LIB")
            .unwrap_or_else(|_| panic!("OPENCV_LIB not set (set it to where the OpenCV libs are)"));
        (PathBuf::from(opencv_dir).join("include"), PathBuf::from(opencv_lib_dir))
    } else {
        // Global configuration for OpenCV build.
        let mut opencv_config = Config::new("opencv");

        // Apply common CMake options.
        cmake_common(&mut opencv_config, &target_os);

        // Set which modules we want to build.
        for module in &all_used_modules {
            opencv_config.define(format!("BUILD_opencv_{}", module), "ON");
        }

        let out_lib_dir = out_dir
            .join("lib")
            .to_str()
            .expect("path must support unicode")
            .replace("\\", "/");

        let out_bin_dir = out_dir
            .join("bin")
            .to_str()
            .expect("path must support unicode")
            .replace("\\", "/");

        // Set various cmake definitions for OpenCV.
        opencv_config
            .define("OPENCV_LIB_INSTALL_PATH", &out_lib_dir)
            .define("OPENCV_3P_LIB_INSTALL_PATH", &out_lib_dir)
            .define("OPENCV_BIN_INSTALL_PATH", &out_bin_dir)
            .define("WITH_CUDA", cmake_bool(feature_cuda))
            // TODO: IPP creates some really annoying build issues on Windows.
            // Eventually we need to fix it.
            .define("WITH_IPP", "OFF")
            .define("BUILD_IPP_IW", "OFF")
            .define("BUILD_opencv_apps", "OFF")
            .define("BUILD_opencv_java_bindings_generator", "OFF")
            .define("BUILD_opencv_js", "OFF")
            .define("BUILD_opencv_python_bindings_generator", "OFF")
            .define("BUILD_opencv_world", "OFF")
            .define("INSTALL_CREATE_DISTRIB", "ON")
            .define("INSTALL_PYTHON_EXAMPLES", "OFF")
            .define("INSTALL_C_EXAMPLES", "OFF")
            .define("OPENCV_ENABLE_NONFREE", "ON")
            .define("OPENCV_EXTRA_MODULES_PATH", contrib_modules_path.into_os_string())
            .define("BUILD_opencv_python", "OFF")
            .define("BUILD_opencv_python2", "OFF")
            .define("BUILD_opencv_python3", "OFF")
            .define("BUILD_TESTS", "OFF")
            .define("BUILD_PERF_TESTS", "OFF")
            .define("BUILD_DOCS", "OFF")
            .define("BUILD_EXAMPLES", "OFF");

        let dst = opencv_config.build();

        println!("cargo:rustc-link-lib=static=cvsys");

        (dst.join("include"), dst.join("lib"))
    };

    // Global configuration for our native wrapper called cvsys.
    let mut cvsys_config = Config::new(".");

    // Apply common CMake options.
    cmake_common(&mut cvsys_config, &target_os);

    // Set modules used and opencv include dir for cvsys.
    cvsys_config
        .define("CV_CORE_MODULES", used_core_modules.join(";"))
        .define("CV_CONTRIB_MODULES", used_contrib_modules.join(";"))
        .define("CVSYS_INCLUDE_DIR", &opencv_include_dir);

    // Build cvsys.
    let dst = cvsys_config.build();

    // Link cvsys.
    println!("cargo:rustc-link-lib=static=cvsys");

    // Add search path for cvsys.
    println!("cargo:rustc-link-search={}", dst.join("lib").display());

    // Link all dependencies.
    for entry in opencv_lib_dir.read_dir()? {
        let entry = entry?;
        if entry.path().extension().map(|os| os == "cmake").unwrap_or(false) || entry.path().is_dir() {
            continue;
        }
        let libname = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .expect("OpenCV lib names must be unicode")
            .to_owned();
        println!(
            "cargo:rustc-link-lib=static={}",
            if target_os == "windows" {
                &libname
            } else {
                &libname[3..]
            }
        );
    }

    // Add search path for OpenCV libs.
    println!("cargo:rustc-link-search={}", opencv_lib_dir.display());

    // Handle OS-specific linker requirements.
    match target_os.as_str() {
        "linux" => {
            link_package("gtk+-3.0");
            link_package("libpng");
            link_package("zlib");
            println!("cargo:rustc-link-lib=stdc++");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=comdlg32");
            println!("cargo:rustc-link-lib=Vfw32");
            println!("cargo:rustc-link-lib=Ole32");
            println!("cargo:rustc-link-lib=OleAut32");
        }
        // Please send in a PR for your favorite platform!
        _ => {}
    }

    if feature_gen_bindings {
        // Set up bindgen to generate bindings from our C++ wrapper.
        // This whitelists exactly the stuff that is needed using regex.
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

        // Add some common flags.
        let bindings = bindings.clang_args(&["-x", "c++", "-std=c++14"]);

        // Add OpenCV include directory.
        let bindings = bindings.clang_arg(format!("-I{}", opencv_include_dir.display()));

        // Add all wrapper headers.
        let bindings = all_used_modules
            .iter()
            .map(|lib| format!("native/{}.hpp", lib))
            .fold(bindings, Builder::header);

        // Change bindgen settings based on OS.
        let bindings = match target_os.as_str() {
            "linux" => bindings.clang_arg("-stdlib=libc++"),
            _ => bindings,
        };

        // Finally generate the bindings.
        let bindings = bindings.generate().expect("bindgen was unable to generate bindings");

        // Write the bindings in the build directory.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    Ok(())
}
