extern crate gcc;

fn main() {
    let mut opencv_config = gcc::Build::new();
    opencv_config
        .cpp(true)
        .file("native/opencv-wrapper.cc")
        .file("native/utils.cc")
        .include("/usr/local/include")
        .include("native")
        .flag("--std=c++11");

    if cfg!(feature = "gpu") {
        opencv_config.file("native/opencv-gpu.cc");
    }

    opencv_config.compile("libopencv-wrapper.a");

    println!("cargo:rustc-link-search=native=/usr/local/lib");
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
