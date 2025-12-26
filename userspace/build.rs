use std::path::PathBuf;

fn main() {
    let linker_script = PathBuf::from("userspace/linker.ld".to_string());
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
    println!("cargo:rerun-if-changed={}", linker_script.display());
}
