#![cfg(not(windows))] // TODO: <https://gitlab.com/azriel91/autexousious/issues/100>

use assert_cmd::{assert::OutputAssertExt, output::OutputError, Command};
use escargot::CargoBuild;

#[test]
#[ignore] // Can't test on X through CI.
fn start_and_exit() -> Result<(), OutputError> {
    let command = CargoBuild::new()
        .bin("will")
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
