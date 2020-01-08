#![cfg(not(windows))] // TODO: <https://gitlab.com/azriel91/autexousious/issues/100>

use assert_cmd::{assert::OutputAssertExt, output::OutputError, Command};
use escargot::CargoBuild;

#[test]
fn read_and_exit() -> Result<(), OutputError> {
    let command = CargoBuild::new()
        .example("01_read_and_exit")
        .current_release()
        .run()
        .expect("Failed to create `cargo` command")
        .command();
    Command::from_std(command)
        .write_stdin("exit\n")
        .ok()?
        .assert()
        .success();
    Ok(())
}

#[test]
fn read_and_exit_timeout() -> Result<(), OutputError> {
    let command = CargoBuild::new()
        .example("01_read_and_exit")
        .current_release()
        .run()
        .expect("Failed to create `cargo` command")
        .command();

    Command::from_std(command)
        .args(&["-t", "0"])
        .write_stdin("abc\n")
        .ok()?
        .assert()
        .success();
    Ok(())
}
