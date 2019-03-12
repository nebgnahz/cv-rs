extern crate bindgen;
extern crate cmake;

use bindgen::Builder;
use cmake::Config;
use itertools::Itertools;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    // Get profile variables.
    let (configuration, lib_postfix) = match env::var("PROFILE").unwrap().as_str() {
        "debug" => ("Debug", "d"),
        "release" => ("Release", ""),
        _ => panic!("unknown PROFILE env var from Cargo"),
    };

    // Look up the full list of core modules.
    let module_dir_entries = fs::read_dir("opencv/modules")?.collect::<Result<Vec<_>, _>>()?;
    let all_core_modules: Vec<OsString> = module_dir_entries
        .into_iter()
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|ft| ft.is_dir())
                .map(|_| entry.file_name())
        })
        .collect();

    // Look up the full list of core modules.
    let contrib_module_dir_entries = fs::read_dir("opencv_contrib/modules")?.collect::<Result<Vec<_>, _>>()?;
    let all_contrib_modules: Vec<OsString> = contrib_module_dir_entries
        .into_iter()
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|ft| ft.is_dir())
                .map(|_| entry.file_name())
        })
        .collect();

    // All the opencv modules (core and contrib).
    let all_opencv_modules: Vec<String> = all_core_modules
        .iter()
        .chain(all_contrib_modules.iter())
        .map(|s| s.to_str().unwrap().to_owned())
        .collect();

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
    if env::var("CARGO_FEATURE_CUDA").is_ok() {
        used_core_modules.push("cudaobjdetect");
    }

    // All the contrib modules used (core and contrib).
    let contrib_modules = vec!["xfeatures2d", "img_hash", "text"];

    // Collect all the modules used.
    let all_used_modules: Vec<_> = used_core_modules.iter().chain(contrib_modules.iter()).collect();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Path to contrib modules.
    let contrib_modules_path = manifest_dir.join("opencv_contrib").join("modules");
    // Global configuration for OpenCV build.
    let mut config = Config::new(".");

    // Set various cmake definitions.
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
        // TODO: IPP creates some really annoying build issues on Windows.
        // Eventually we need to fix it.
        .define("WITH_IPP", "OFF")
        .define("BUILD_IPP_IW", "OFF")
        .define("BUILD_opencv_apps", "OFF")
        .define("BUILD_opencv_java_bindings_generator", "OFF")
        .define("BUILD_opencv_js", "OFF")
        .define("BUILD_opencv_python_bindings_generator", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", configuration.to_ascii_uppercase())
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
        .define("CV_CORE_MODULES", used_core_modules.join(";"))
        .define("CV_CONTRIB_MODULES", contrib_modules.join(";"));

    // Set which modules we want to build.
    for (var, val) in all_opencv_modules.iter().map(|module| {
        (
            format!("BUILD_opencv_{}", module),
            if all_used_modules.iter().any(|usedm| *usedm == module) { "ON" } else { "OFF" },
        )
    }) {
        config.define(var, val);
    }

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
        .replace("$(Configuration)", configuration)
        .split(';')
        .map(Into::into)
        .collect();

    // This contains the raw lib names.
    let deduped_lib_names: Vec<OsString> = libs_list
        .iter()
        .map(|p| p.file_stem().expect("expected only files in libs list").to_os_string())
        .unique()
        .collect();

    // This contains the lib search paths.
    let deduped_lib_search_paths: Vec<PathBuf> = libs_list
        .iter()
        .map(|p| p.parent().expect("expected libs to be in a directory").to_path_buf())
        .unique()
        .collect();

    // Link cvsys.
    println!(
        "cargo:rustc-link-search={}",
        dst.join("build").join(configuration).display()
    );
    println!("cargo:rustc-link-lib=static=cvsys");

    // Link all cvsys dependencies.
    for libpath in deduped_lib_search_paths {
        println!("cargo:rustc-link-search={}", libpath.display());
    }

    // Add all the static libs that need to be linked to Cargo, adding
    // postfixes as necessary (on Windows a 'd' is appended to libs in debug mode).
    for libname in deduped_lib_names {
        println!(
            "cargo:rustc-link-lib=static={}{}",
            libname.to_str().expect("OpenCV lib names must be valid UTF-8"),
            lib_postfix
        );
    }

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

    // Add core module includes.
    let bindings = bindings.clang_args(used_core_modules.iter().map(|lib| {
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

    // Add contrib module includes.
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

    // Add include from root of build folder.
    let bindings = bindings.clang_arg(format!("-I{}", dst.join("include").display()));

    // Add all wrapper headers.
    let bindings = all_used_modules
        .iter()
        .map(|lib| format!("native/{}.hpp", lib))
        .fold(bindings, Builder::header);

    // Finally generate the bindings.
    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings in the build directory.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
