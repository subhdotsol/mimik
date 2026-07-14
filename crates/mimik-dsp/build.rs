fn main() {
    println!("cargo:rustc-link-lib=rubberband");
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rerun-if-changed=build.rs");
}
