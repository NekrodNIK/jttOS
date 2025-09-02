use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/entry.nasm");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let asm_file = "src/entry.nasm";
    let obj_file = out_dir.join("entry.o");
    let lib_file = out_dir.join("libentry.a");

    // Assemble the NASM file
    let status = Command::new("nasm")
        .arg("-f")
        .arg("elf32") // Use 'win64' for Windows, 'macho64' for macOS
        .arg("-o")
        .arg(&obj_file)
        .arg(asm_file)
        .status()
        .expect("Failed to execute nasm");

    if !status.success() {
        panic!("NASM assembly failed");
    }

    // Create a static library from the object file
    let status = Command::new("ar")
        .arg("rcs")
        .arg(&lib_file)
        .arg(&obj_file)
        .status()
        .expect("Failed to execute ar");

    if !status.success() {
        panic!("Failed to create static library");
    }

    // Tell Cargo where to find the library
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=entry");

    let linker_script = PathBuf::from(format!("linker.ld"));

    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
    println!("cargo:rerun-if-changed={}", linker_script.display());
}
