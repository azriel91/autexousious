extern crate assert_cmd;

use assert_cmd::{cargo::CargoError, prelude::*};
use std::process::Command;

#[test]
fn start_and_exit() -> Result<(), CargoError> {
    Command::main_binary()?
        .env("APP_DIR", env!("CARGO_MANIFEST_DIR"))
        .with_stdin()
        .buffer("exit\n")
        .output()
        .unwrap()
        .assert()
        .success();
    Ok(())
}
