#![cfg(not(windows))] // TODO: <https://gitlab.com/azriel91/autexousious/issues/100>

use assert_cmd::{
    assert::OutputAssertExt,
    cmd::{OutputError, OutputOkExt},
    stdin::CommandStdInExt,
};
use escargot::CargoBuild;

#[test]
fn read_and_exit() -> Result<(), OutputError> {
    CargoBuild::new()
        .example("01_read_and_exit")
        .current_release()
        .run()
        .expect("Failed to create `cargo` command")
        .command()
        .with_stdin()
        .buffer("exit\n")
        .ok()?
        .assert()
        .success();
    Ok(())
}

#[test]
fn read_and_exit_timeout() -> Result<(), OutputError> {
    CargoBuild::new()
        .example("01_read_and_exit")
        .current_release()
        .run()
        .expect("Failed to create `cargo` command")
        .command()
        .args(&["-t", "0"])
        .with_stdin()
        .buffer("abc\n")
        .ok()?
        .assert()
        .success();
    Ok(())
}
