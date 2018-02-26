extern crate cc;
extern crate cmake;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[cfg(windows)]
extern crate winreg;

use std::fs::File;
use std::io::Read;
use std::path::*;
use std::env;
use std::process::Command;

const IS_CUDA_ENABLED: bool = cfg!(feature = "cuda");
#[cfg(all(windows, target_env = "gnu"))]
const BINARY_NAME: &str = "libopencv_world340.dll";
#[cfg(all(windows, target_env = "msvc"))]
const BINARY_NAME: &str = "opencv_world340.dll";
#[cfg(unix)]
const BINARY_NAME: &str = "opencv_version";

fn main() {
    if !Path::new("opencv/.git").exists() || !Path::new("opencv_contrib/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
    }

    let config = read_file("build.toml");
    let config: BuildConfig = toml::from_str(&config).unwrap();

    let install_path = build_opencv_and_get_path(&config);

    let files = get_files("native");

    let mut opencv_config = cc::Build::new();
    opencv_config
        .cpp(true)
        .files(files)
        .include("native")
        .include(install_path.join("include"));

    if cfg!(not(target_env = "msvc")) {
        opencv_config.flag("--std=c++11");
    }

    if IS_CUDA_ENABLED {
        let cuda_files = get_files("native/cuda");
        opencv_config.files(cuda_files);
    }

    opencv_config.compile("libopencv-wrapper.a");
    opencv_link(&config);
}

fn build_opencv_and_get_path(config: &BuildConfig) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let compiler = get_compiler(&config);
    let compiler_prefix = get_prefix(&config);

    let install_prefix = current_dir.join("artifacts").join(compiler_prefix);

    let (opencv_binary, _) = get_bin_and_lib(config);
    if !opencv_binary.exists() {
        let extra_modules_path = current_dir.join("opencv_contrib").join("modules");

        std::fs::create_dir_all(&install_prefix).unwrap();

        let arguments = [
            ("WITH_CUDA", if IS_CUDA_ENABLED { "ON" } else { "OFF" }),
            ("CUDA_ARCH_BIN", "5.2"),
            ("CUDA_ARCH_PTX", ""),
            ("BUILD_opencv_java", "OFF"),
            ("BUILD_opencv_python", "OFF"),
            ("BUILD_opencv_python2", "OFF"),
            ("BUILD_opencv_python3", "OFF"),
            ("BUILD_TESTS", "OFF"),
            ("BUILD_PERF_TESTS", "OFF"),
            ("BUILD_DOCS", "OFF"),
            ("BUILD_EXAMPLES", "OFF"),
            ("INSTALL_CREATE_DISTRIB", "ON"),
            (
                "OPENCV_EXTRA_MODULES_PATH",
                extra_modules_path.to_str().unwrap(),
            ),
            ("CMAKE_SH", "CMAKE_SH-NOTFOUND"),
        ];

        let cpu_count = num_cpus::get();
        let mut config = cmake::Config::new("opencv");
        config
            .out_dir(&install_prefix)
            .env("NUM_JOBS", cpu_count.to_string())
            .profile("Release");

        if let Some(compiler) = compiler {
            config.generator(compiler);
        }
        for &(k, v) in arguments.iter() {
            config.define(k, v);
        }

        config.build();
    }
    post_build(&opencv_binary);
    install_prefix
}

#[cfg(all(windows, target_env = "msvc"))]
fn get_compiler(config: &BuildConfig) -> Option<&'static str> {
    let result = match config.vc_compiler {
        Compiler::VC14 => "Visual Studio 14 2015 Win64",
        Compiler::VC15 => {
            if IS_CUDA_ENABLED {
                eprint!("Cuda is compatible with VC14 only. Please, change compiler");
                std::process::exit(0x0100);
            }
            "Visual Studio 15 2017 Win64"
        }
    };
    Some(result)
}

#[cfg(all(windows, target_env = "gnu"))]
fn get_compiler(_: &BuildConfig) -> Option<&'static str> {
    Some("MinGW Makefiles")
}

#[cfg(unix)]
fn get_compiler(_: &BuildConfig) -> Option<&'static str> {
    None
}

#[cfg(all(windows, target_env = "msvc"))]
fn get_prefix(config: &BuildConfig) -> &'static str {
    match config.vc_compiler {
        Compiler::VC14 if IS_CUDA_ENABLED => "vc14_cuda",
        Compiler::VC14 => "vc14",
        Compiler::VC15 => "vc15",
    }
}

#[cfg(all(windows, target_env = "gnu"))]
fn get_prefix(_: &BuildConfig) -> &'static str {
    "mingw"
}

#[cfg(unix)]
fn get_prefix(_: &BuildConfig) -> &'static str {
    if IS_CUDA_ENABLED {
        "default_cuda"
    } else {
        "default"
    }
}

fn get_bin_and_lib(config: &BuildConfig) -> (PathBuf, PathBuf) {
    let prefix = get_prefix(config);
    let target_dir = env::current_dir().unwrap().join("artifacts").join(prefix);

    let target_dir = if cfg!(windows) {
        target_dir.join("x64").join(prefix)
    } else {
        target_dir
    };

    (
        target_dir.join("bin").join(BINARY_NAME),
        target_dir.join("lib"),
    )
}

fn opencv_link(config: &BuildConfig) {
    let (_, lib) = get_bin_and_lib(config);
    if let Err(e) = try_opencv_link(&lib) {
        eprint!("Error while building cv-rs: {:?}.", e);
        std::process::exit(0x0100);
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct BuildConfig {
    #[cfg(target_env = "msvc")] vc_compiler: Compiler,
}

#[derive(Debug, Clone, Copy, Deserialize)]
enum Compiler {
    VC14,
    VC15,
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn get_files(path: &str) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok().map(|x| x.path()))
        .filter(|x| x.extension().map(|e| e == "cc").unwrap_or(false))
        .collect::<Vec<_>>()
}

fn try_opencv_link(opencv_dir: &PathBuf) -> Result<(), Box<std::error::Error>> {
    let files = std::fs::read_dir(opencv_dir)?;
    let opencv_world_entry = files.filter_map(|entry| entry.ok()).find(|entry| {
        let file_name = entry.file_name().to_string_lossy().into_owned();
        (file_name.starts_with("opencv_world") || file_name.starts_with("libopencv_world"))
            && !file_name.ends_with("d.lib")
    });
    match opencv_world_entry {
        Some(opencv_world) => {
            let opencv_world = opencv_world.file_name();
            let opencv_world = opencv_world.into_string().unwrap();
            let opencv_world_without_extension: String = opencv_world.chars().take_while(|c| *c != '.').collect();
            println!(
                "cargo:rustc-link-search=native={}",
                opencv_dir.to_str().unwrap()
            );
            println!("cargo:rustc-link-lib={}", opencv_world_without_extension);
            Ok(())
        }
        None => panic!(format!(
            "Cannot find opencv_world file in '{:?}'",
            opencv_dir
        )),
    }
}

#[cfg(windows)]
fn post_build(opencv_binary: &PathBuf) {
    let bin_path = opencv_binary.parent().unwrap();
    let bin_path = bin_path.to_str().unwrap();
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let environment = hkcu.open_subkey("Environment").unwrap();
    let path: String = environment.get_value("Path").unwrap();
    if !path.contains(bin_path) {
        let new_path = format!("{};{}", bin_path, path);
        let _output = Command::new("setx")
            .args(&["PATH", &new_path])
            .status()
            .unwrap();
    }
}

#[cfg(unix)]
fn post_build(_: &PathBuf) {
    //    let bin_path = Path::new("~/Documents/cv-rs/artifacts/default");
    //    let lib = bin_path.join("lib");
    //    let include = bin_path.join("include");
    //    run_command("cp", &["-r", ])

}
//
//fn run_command(name: &str, args: &[&str]) {
//    Command::new(name)
//        .args(args)
//        .status()
//        .unwrap();
//}
