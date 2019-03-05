#![cfg(not(windows))] // TODO: <https://gitlab.com/azriel91/autexousious/issues/100>

use assert_cmd::{
    assert::OutputAssertExt,
    cmd::{OutputError, OutputOkExt},
};
use escargot::CargoBuild;

#[test]
fn example_01_draw_text() -> Result<(), OutputError> {
    CargoBuild::new()
        .example("01_draw_text")
        .current_release()
        .run()
        .expect("Failed to create `cargo` command")
        .command()
        .env("APP_DIR", env!("CARGO_MANIFEST_DIR"))
        .args(&["--timeout", "0"])
        .ok()?
        .assert()
        .success();

    Ok(())
}
