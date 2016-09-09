extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .file("native/opencv-wrapper.cc")
        .include("/usr/local/include")
        .include("native")
        .flag("--std=c++11")
        .compile("libopencv-wrapper.a");

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=opencv_core");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=opencv_imgproc");
    println!("cargo:rustc-link-lib=opencv_highgui");
    println!("cargo:rustc-link-lib=opencv_videoio");
    println!("cargo:rustc-link-lib=opencv_objdetect");
    println!("cargo:rustc-link-lib=opencv_video");
}
