

use assert_cmd::{cargo::CargoError, prelude::*};
use std::process::Command;

#[test]
fn example_01_menu() -> Result<(), CargoError> {
    Command::cargo_example("01_menu")?
        .env("APP_DIR", env!("CARGO_MANIFEST_DIR"))
        .args(&["--timeout", "0"])
        .output()
        .unwrap()
        .assert()
        .success();
    Ok(())
}
