use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let boot_src = "src/boot.nasm";
    println!("cargo:rerun-if-changed={}", boot_src);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let boot_obj = out_dir.join("boot.o");
    let boot_lib = out_dir.join("libboot.a");

    let status = Command::new("nasm")
        .arg("-f")
        .arg("elf32")
        .arg("-o")
        .arg(&boot_obj)
        .arg(boot_src)
        .status()
        .expect("Failed to execute nasm");

    if !status.success() {
        panic!("Nasm failed with status {}", status.success());
    }

    let status = Command::new("ar")
        .arg("rcs")
        .arg(&boot_lib)
        .arg(&boot_obj)
        .status()
        .expect("Failed to execute ar");

    if !status.success() {
        panic!("Ar failed with status {}", status.success());
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=boot");

    let linker_script = PathBuf::from(format!("linker.ld"));

    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
    println!("cargo:rerun-if-changed={}", linker_script.display());
}
