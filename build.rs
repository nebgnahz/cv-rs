extern crate cc;

#[cfg(windows)]
mod windows {
    use std::error::Error;
    use std::{env, fmt, fs, io, process};

    pub fn opencv_include() -> String {
        if let Ok(dir) = env::var("OPENCV_DIR") {
            format!("{}\\include", dir)
        } else {
            eprint!("%OPENCV_DIR% is not set.");
            process::exit(0x0100);
        }
    }

    pub fn opencv_link() {
        if let Err(e) = try_opencv_link() {
            eprint!("Error while building cv-rs: {:?}.", e);
            process::exit(0x0100);
        }
    }

    fn try_opencv_link() -> Result<(), Box<Error>> {
        let opencv_dir = env::var("OPENCV_LIB")?;
        let files = fs::read_dir(&opencv_dir)?.collect::<Vec<_>>();
        let opencv_world = get_opencv_lib_path(files.iter(), "world")?;
        let img_hash = get_opencv_lib_path(files.iter(), "img_hash")?;

        println!("cargo:rustc-link-search=native={}", opencv_dir);
        println!("cargo:rustc-link-lib={}", opencv_world);
        println!("cargo:rustc-link-lib={}", img_hash);
        Ok(())
    }

    fn get_opencv_lib_path<'a, T: Iterator<Item = &'a io::Result<fs::DirEntry>>>(
        files: T,
        name: &str,
    ) -> Result<String, Box<Error>> {
        let opencv_world_entry = files.filter_map(|entry| entry.as_ref().ok()).find(|entry| {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            (file_name.starts_with(&format!("opencv_{}", name))
                || file_name.starts_with(&format!("libopencv_{}", name)))
                && !file_name.ends_with("d.lib")
        });
        let lib = opencv_world_entry.ok_or_else(|| {
            BuildError::new(format!(
                "Cannot find opencv_{} file in provided %OPENCV_LIB% directory",
                name
            ))
        })?;
        let lib = lib.file_name();
        let lib = lib
            .into_string()
            .map_err(|e| BuildError::new(format!("Cannot convert path '{:?}' to string", e)))?;
        // we expect filename to be something like 'open_world340.lib' or
        // 'open_world.340.dll.a', so we just consider everything after the
        // version number is an extension
        let lib_without_extension = lib.trim_right_matches(|c: char| !c.is_numeric());
        Ok(lib_without_extension.into())
    }

    #[derive(Debug)]
    struct BuildError {
        details: String,
    }

    impl BuildError {
        fn new<T: Into<String>>(details: T) -> Self {
            Self {
                details: details.into(),
            }
        }
    }

    impl Error for BuildError {
        fn description(&self) -> &str {
            &self.details
        }
    }

    impl fmt::Display for BuildError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }
}

#[cfg(unix)]
mod unix {
    pub fn opencv_include() -> &'static str {
        "/usr/local/include"
    }

    pub fn opencv_link() {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=opencv_core");
        println!("cargo:rustc-link-lib=opencv_features2d");
        println!("cargo:rustc-link-lib=opencv_xfeatures2d");
        println!("cargo:rustc-link-lib=opencv_highgui");
        println!("cargo:rustc-link-lib=opencv_img_hash");
        println!("cargo:rustc-link-lib=opencv_imgcodecs");
        println!("cargo:rustc-link-lib=opencv_imgproc");
        println!("cargo:rustc-link-lib=opencv_optflow");
        println!("cargo:rustc-link-lib=opencv_objdetect");
        println!("cargo:rustc-link-lib=opencv_text");
        println!("cargo:rustc-link-lib=opencv_videoio");
        println!("cargo:rustc-link-lib=opencv_video");
        if cfg!(feature = "cuda") {
            println!("cargo:rustc-link-lib=opencv_cudaobjdetect");
        }
    }
}

#[cfg(windows)]
use windows::*;

#[cfg(unix)]
use unix::*;

fn main() {
    let files = get_files("native");

    let mut opencv_config = cc::Build::new();
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

fn get_files(path: &str) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok().map(|x| x.path()))
        .filter(|x| x.extension().map(|e| e == "cc").unwrap_or(false))
        .collect::<Vec<_>>()
}
