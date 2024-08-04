use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("gcc")
        .args(&["-c", "-g", "awesome.c", "-o"])
        .arg(&format!("{}/awesome.o", out_dir))
        .status()
        .expect("Failed to compile C code");

    if !status.success() {
        panic!("gcc failed to compile awesome.c");
    }

    let status = Command::new("ar")
        .args(&["rcs", "libawesome.a", "awesome.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .expect("Failed to create static library");

    if !status.success() {
        panic!("ar failed to create static library");
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=awesome");
}
