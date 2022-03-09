use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["src/malloc.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/malloc.o", out_dir))
        .status().unwrap();
    Command::new("gcc")
        .args(&["src/util.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/util.o", out_dir))
        .status().unwrap();
    Command::new("ar")
        .args(&["crus", "libdlmalloc.a", "malloc.o", "util.o"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=dlmalloc");
    println!("cargo:rerun-if-changed=src/malloc.c");
    println!("cargo:rerun-if-changed=src/util.c");
}
