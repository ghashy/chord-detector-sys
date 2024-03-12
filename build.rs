use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut logfile = File::create("build.log").unwrap();
    logfile
        .write("Building detector-lib-bridge library...\n".as_bytes())
        .unwrap();

    let out_dir = "lib/build";

    logfile
        .write_fmt(format_args!(
            "Current dir: {}\n",
            std::env::current_dir().unwrap().display()
        ))
        .unwrap();

    // Create output directory and configure cmake
    if !PathBuf::from(out_dir).exists() {
        match Command::new("mkdir").args(&[out_dir]).status() {
            Ok(status) => {
                logfile
                    .write_fmt(format_args!(
                        "mkdir command exited with status: {}\n",
                        status
                    ))
                    .unwrap();
            }
            Err(e) => panic!("error: {}", e),
        }
        match Command::new("cmake")
            .args(&[
                "-Slib",
                "-Blib/build",
                "-DBUILD_SHARED_LIBS=OFF",
                "-DLINK_WITH_STATIC_LIBRARIES=ON",
            ])
            .status()
        {
            Ok(status) => {
                logfile
                    .write_fmt(format_args!(
                        "cmake configuration command exited with status: {}\n",
                        status
                    ))
                    .unwrap();
            }
            Err(e) => panic!("error: {}", e),
        }
    }

    match Command::new("cmake").args(&["--build", out_dir, "-j8"]).status() {
        Ok(status) => {
            logfile
                .write_fmt(format_args!(
                    "cmake build command exited with status: {}\n",
                    status
                ))
                .unwrap();
        }
        Err(e) => panic!("error: {}", e),
    }

    let out_path = fs::canonicalize(&out_dir).unwrap();
    // Std c++ lib
    println!("cargo:rustc-flags=-l dylib=c++");
    // Search for libs
    println!(r"cargo:rustc-link-search={}/detector-lib", out_path.display());
    println!(r"cargo:rustc-link-search={}", out_path.display());
    // Link libs
    println!(r"cargo:rustc-link-lib=static={}", "detector-lib-bridge");
    println!(r"cargo:rustc-link-lib=static={}", "detector-lib");

    println!("cargo:rerun-if-changed=lib/source");
}
