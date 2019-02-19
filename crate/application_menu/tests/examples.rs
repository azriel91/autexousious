use assert_cmd::{
    assert::OutputAssertExt,
    cmd::{OutputError, OutputOkExt},
};
use escargot::CargoBuild;

#[test]
fn example_01_menu() -> Result<(), OutputError> {
    CargoBuild::new()
        .example("01_menu")
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
