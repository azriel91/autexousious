#! /usr/bin/env run-cargo-script
//! Compiles sources, tests, and examples in a crate.
//!
//! ```cargo
//! [dependencies]
//! structopt = "0.2.5"
//! structopt-derive = "0.2.5"
//! ```

#[macro_use]
extern crate structopt;

use std::process::Command;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "build_compile",
            about = "Compiles crates, binaries, and examples in a workspace.")]
struct Opt {
    #[structopt(long = "release", help = "Use the release profile.")]
    release: bool,
}

/// Compiles this crate's sources.
///
/// This is distinct from `cargo test` because crates that do not have tests will not have its
/// sources compiled.
///
/// # Parameters
///
/// * `release`: Whether compilation should use the release profile.
fn compile_sources(release: bool) {
    compile_crate(release, vec!["build"]);
}

/// Compiles this crate's tests.
///
/// # Parameters
///
/// * `release`: Whether compilation should use the release profile.
fn compile_tests(release: bool) {
    compile_crate(release, vec!["test", "--no-run"]);
}

/// Compiles this crate's examples.
///
/// # Parameters
///
/// * `release`: Whether compilation should use the release profile.
fn compile_examples(release: bool) {
    compile_crate(release, vec!["build", "--examples"]);
}

/// Compiles all crates using the specific subcommand.
///
/// # Parameters
///
/// * `release`: Whether compilation should use the release profile.
/// * `subcommand`: The cargo subcommand to use for compilation
fn compile_crate(release: bool, base_args: Vec<&'static str>) {
    let mut args = base_args;
    if release {
        args.push("--release");
    }

    let mut command = Command::new("cargo");
    command.args(&args);
    println!("Running command: `{:?}`", &command);

    let mut child = command.spawn().expect("Failed to spawn command.");
    let exit_status = child.wait().expect("Failed to wait on child process.");

    if !exit_status.success() {
        panic!("Failed to execute command.");
    }
}

fn main() {
    let opt = Opt::from_args();

    compile_sources(opt.release);
    compile_tests(opt.release);
    compile_examples(opt.release);
}
