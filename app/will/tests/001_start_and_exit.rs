#![cfg(not(windows))] // TODO: <https://gitlab.com/azriel91/autexousious/issues/100>

use assert_cmd::{
    assert::OutputAssertExt,
    cmd::{OutputError, OutputOkExt},
    stdin::CommandStdInExt,
};
use escargot::CargoBuild;

#[test]
#[ignore] // Can't test on X through CI.
fn start_and_exit() -> Result<(), OutputError> {
    CargoBuild::new()
        .bin("will")
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
