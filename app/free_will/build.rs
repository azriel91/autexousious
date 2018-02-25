extern crate application;

use std::env;
use std::io;
#[cfg(unix)]
use std::os::unix::fs;
#[cfg(windows)]
use std::os::windows::fs;
use std::path::Path;

use application::resource::dir;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let resources_dir = Path::new(&crate_dir).join(dir::RESOURCES);
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_resources_dir = Path::new(&out_dir).join(dir::RESOURCES);

    let message = format!(
        "Failed to create symlink for '{}' at '{}'",
        resources_dir.display(),
        target_resources_dir.display()
    );
    if !target_resources_dir.exists() {
        create_symlink(&resources_dir, &target_resources_dir).expect(&message);
    }
}

#[cfg(unix)]
fn create_symlink(target: &AsRef<Path>, symlink_path: &AsRef<Path>) -> io::Result<()> {
    fs::symlink(target, symlink_path)
}

#[cfg(windows)]
fn create_symlink(target: &AsRef<Path>, symlink_path: &AsRef<Path>) -> io::Result<()> {
    fs::symlink_dir(target, symlink_path)
}
