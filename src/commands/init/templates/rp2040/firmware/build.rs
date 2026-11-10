//! Puts `memory.x` on the linker search path and relinks when it changes.

use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"));
    std::fs::write(out.join("memory.x"), include_bytes!("memory.x")).expect("OUT_DIR is writable");
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
