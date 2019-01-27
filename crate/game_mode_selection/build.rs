use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Enable "nightly" feature on nightly channel.
    if let Channel::Nightly = version_meta()
        .expect("Failed to read rustc version")
        .channel
    {
        println!("cargo:rustc-cfg=nightly");
    }
}
