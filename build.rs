extern crate gcc;

#[cfg(windows)]
fn opencv_include() -> String {
    if let Ok(dir) = std::env::var("OPENCV_DIR") {
        format!("{}\\include", dir)
    } else {
        eprint!("%OPENCV_DIR% is not set properly.");
        std::process::exit(0x0100);
    }
}

#[cfg(windows)]
fn opencv_lib() -> &'static str {
    if let Ok(dir) = std::env::var("OPENCV_DIR") {
        format!("{}\\lib", dir)
    } else {
        eprint!("%OPENCV_DIR% is not set properly.");
        std::process::exit(0x0100);
    }
}

#[cfg(unix)]
fn opencv_include() -> &'static str {
    "/usr/local/include"
}

#[cfg(unix)]
fn opencv_lib() -> &'static str {
    "/usr/local/lib"
}

fn main() {
    let mut opencv_config = gcc::Build::new();
    opencv_config
        .cpp(true)
        .file("native/opencv-wrapper.cc")
        .file("native/utils.cc")
        .include("native")
        .include(opencv_include());

    if cfg!(feature = "gpu") {
        opencv_config.file("native/opencv-gpu.cc");
    }

    opencv_config.compile("libopencv-wrapper.a");

    println!("cargo:rustc-link-search=native={}", opencv_lib());
    println!("cargo:rustc-link-lib=opencv_core");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=opencv_imgproc");
    println!("cargo:rustc-link-lib=opencv_highgui");
    println!("cargo:rustc-link-lib=opencv_videoio");
    println!("cargo:rustc-link-lib=opencv_objdetect");
    println!("cargo:rustc-link-lib=opencv_video");

    if cfg!(feature = "gpu") {
        println!("cargo:rustc-link-lib=opencv_cudaobjdetect");
    }
}
