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
fn opencv_link() {
    if let Ok(dir) = std::env::var("OPENCV_LIB") {
        if let Ok(mut files) = std::fs::read_dir(&dir) {
            let opencv_world_entry = files
                .filter_map(|entry| entry.ok())
                .find(|entry| {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    file_name.starts_with("opencv_world") && !file_name.ends_with("d.lib")
                });
            if let Some(opencv_world) = opencv_world_entry {
                let opencv_world = opencv_world.path();
                println!("cargo:rustc-link-search=native={}", dir);
                println!("cargo:rustc-link-lib={}", opencv_world.file_stem().unwrap().to_string_lossy());
                return;
            }
        }
        eprint!("Cannot find opencv_world file in provided %OPENCV_DIR%");
        std::process::exit(0x0100);
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
fn opencv_link() {
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

fn main() {
    let mut opencv_config = gcc::Build::new();
    opencv_config
        .cpp(true)
        .file("native/opencv-wrapper.cc")
        .file("native/utils.cc")
        .include("native")
        .include(opencv_include())
        .flag("--std=c++11");

    if cfg!(feature = "gpu") {
        opencv_config.file("native/opencv-gpu.cc");
    }

    opencv_config.compile("libopencv-wrapper.a");
    opencv_link();
}
