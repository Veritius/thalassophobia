//! This build script is used in all crates via symlinks.

fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/");
}