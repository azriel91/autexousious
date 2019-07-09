use assert_cmd::assert::OutputAssertExt;
use escargot::{error::CargoError, CargoBuild};

#[test]
#[ignore] // Not necessary in CI.
fn example_01_menu() -> Result<(), CargoError> {
    CargoBuild::new()
        .example("01_menu")
        .current_release()
        .run()?
        .command()
        .env("APP_DIR", env!("CARGO_MANIFEST_DIR"))
        .args(&["--timeout", "0"])
        .assert()
        .success();

    Ok(())
}
