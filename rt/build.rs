use cc::Build;
use std::io::Write;

fn main() {
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("link.ld"))
        .unwrap()
        .write_all(include_bytes!("link.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    Build::new().file("src/asm.s").compile("asm");

    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rerun-if-changed=src/asm.s");
}
