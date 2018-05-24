extern crate gcc;

#[cfg(windows)]
fn opencv_include() -> String {
    if let Ok(dir) = std::env::var("OPENCV_DIR") {
        format!("{}\\include", dir)
    } else {
        eprint!("%OPENCV_DIR% is not set.");
        std::process::exit(0x0100);
    }
}

#[cfg(windows)]
fn opencv_link() {
    if let Err(e) = try_opencv_link() {
        eprint!("Error while building cv-rs: {:?}.", e);
        std::process::exit(0x0100);
    }
}

#[cfg(windows)]
fn try_opencv_link() -> Result<(), Box<std::error::Error>> {
    let opencv_dir = std::env::var("OPENCV_LIB")?;
    let files = std::fs::read_dir(&opencv_dir)?;
    let opencv_world_entry = files.filter_map(|entry| entry.ok()).find(|entry| {
        let file_name = entry.file_name().to_string_lossy().into_owned();
        (file_name.starts_with("opencv_world") || file_name.starts_with("libopencv_world"))
            && !file_name.ends_with("d.lib")
    });
    match opencv_world_entry {
        Some(opencv_world) => {
            let opencv_world = opencv_world.file_name();
            let opencv_world = opencv_world.into_string().unwrap();
            // we expect filename to be something like 'open_world340.lib' or
            // 'open_world.340.dll.a', so we just consider everything after the
            // version number is an extension
            let opencv_world_without_extension = opencv_world.trim_right_matches(|c: char| !c.is_numeric());
            println!("cargo:rustc-link-search=native={}", opencv_dir);
            println!("cargo:rustc-link-lib={}", opencv_world_without_extension);
            Ok(())
        }
        None => Err(Box::new(BuildError {
            details: "Cannot find opencv_world file in provided %OPENCV_LIB% directory",
        })),
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
    println!("cargo:rustc-link-lib=opencv_features2d");
    println!("cargo:rustc-link-lib=opencv_xfeatures2d");
    println!("cargo:rustc-link-lib=opencv_highgui");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=opencv_imgproc");
    println!("cargo:rustc-link-lib=opencv_objdetect");
    println!("cargo:rustc-link-lib=opencv_text");
    println!("cargo:rustc-link-lib=opencv_videoio");
    println!("cargo:rustc-link-lib=opencv_video");
    if cfg!(feature = "cuda") {
        println!("cargo:rustc-link-lib=opencv_cudaobjdetect");
    }
}

fn main() {
    let files = get_files("native");

    let mut opencv_config = gcc::Build::new();
    opencv_config
        .cpp(true)
        .files(files)
        .include("native")
        .include(opencv_include());

    if cfg!(not(target_env = "msvc")) {
        opencv_config.flag("--std=c++11");
    }

    if cfg!(feature = "cuda") {
        let cuda_files = get_files("native/cuda");
        opencv_config.files(cuda_files);
    }

    opencv_config.compile("libopencv-wrapper.a");
    opencv_link();
}

#[cfg(windows)]
#[derive(Debug)]
struct BuildError {
    details: &'static str,
}

#[cfg(windows)]
impl std::error::Error for BuildError {
    fn description(&self) -> &str {
        self.details
    }
}

#[cfg(windows)]
impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

fn get_files(path: &str) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok().map(|x| x.path()))
        .filter(|x| x.extension().map(|e| e == "cc").unwrap_or(false))
        .collect::<Vec<_>>()
}
