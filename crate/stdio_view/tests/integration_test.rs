extern crate assert_cmd;

use assert_cmd::{cargo::CargoError, prelude::*};
use std::process::Command;

#[test]
fn read_and_exit() -> Result<(), CargoError> {
    Command::cargo_example("01_read_and_exit")?
        .with_stdin()
        .buffer("exit\n")
        .output()
        .unwrap()
        .assert()
        .success();
    Ok(())
}

#[test]
fn read_and_exit_timeout() -> Result<(), CargoError> {
    Command::cargo_example("01_read_and_exit")?
        .args(&["-t", "0"])
        .with_stdin()
        .buffer("abc\n")
        .output()
        .unwrap()
        .assert()
        .success();
    Ok(())
}
