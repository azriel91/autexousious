#! /usr/bin/env run-cargo-script
//! Compiles sources, tests, and examples in a crate.
//!
//! ```cargo
//! [dependencies]
//! cargo_metadata = "0.5.3"
//! structopt = "0.2.5"
//! structopt-derive = "0.2.5"
//! ```

#[macro_use]
extern crate structopt;
extern crate cargo_metadata;

use std::path::{Path, PathBuf};
use std::process::Command;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "build_compile",
            about = "Compiles crates, binaries, and examples in a workspace.",
            raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    #[structopt(long = "manifest-path", help = "Path to the workspace Cargo.toml",
                parse(from_os_str))]
    manifest_path: Option<PathBuf>,

    // Pass the rest of the arguments through
    #[structopt(help = "Command to run inside each crate directory.",
                raw(allow_hyphen_values = "true"))]
    command: Vec<String>,
}

fn run_command_in(path: &Path, command_string: &[String]) {
    println!("Crate directory: {}", path.display());

    let mut command = Command::new(&command_string[0]);
    command.args(&command_string[1..]).current_dir(path);

    let mut child = command.spawn().expect("Failed to spawn command.");
    let exit_status = child.wait().expect("Failed to wait on child process.");

    if !exit_status.success() {
        panic!("Failed to execute command.");
    }
}

fn main() {
    let opt = Opt::from_args();
    let metadata = cargo_metadata::metadata(opt.manifest_path.as_ref().map(|p| p.as_path()))
        .expect("Failed to read workspace metadata.");

    metadata
        .packages
        .iter()
        .filter_map(|package| Path::new(&package.manifest_path).parent())
        .for_each(|crate_dir| run_command_in(&crate_dir, &opt.command));
}
