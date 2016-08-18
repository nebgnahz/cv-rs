extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .file("native/opencv-wrapper.cc")
        .include("native")
        .compile("libopencv-wrapper.a");

    println!("cargo:rustc-link-search=native=-L/usr/local/lib");
    println!("cargo:rustc-link-lib=opencv_core");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
}
