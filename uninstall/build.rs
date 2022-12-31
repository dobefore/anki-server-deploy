use std::env;
fn main() {
    // out dir is in target/release/build
     let out_dir = env::var("OUT_DIR").unwrap();
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/press.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/press.c")
        .compile("press");

            println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=press");
}