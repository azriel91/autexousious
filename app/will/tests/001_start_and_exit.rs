use assert_cmd::{
    assert::OutputAssertExt,
    cmd::{OutputError, OutputOkExt},
    stdin::CommandStdInExt,
};
use escargot::CargoBuild;

#[test]
fn start_and_exit() -> Result<(), OutputError> {
    CargoBuild::new()
        .bin("will")
        .run()
        .expect("Failed to create `cargo` command")
        .command()
        .env("APP_DIR", env!("CARGO_MANIFEST_DIR"))
        .with_stdin()
        .buffer("exit\n")
        .ok()?
        .assert()
        .success();

    Ok(())
}
