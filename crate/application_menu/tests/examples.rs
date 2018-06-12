extern crate assert_cli;

#[test]
fn example_01_menu() {
    // We need to inherit the environment, otherwise we won't be able to run `cargo`. See
    // <https://github.com/assert-rs/assert_cli/issues/58>
    let environment =
        assert_cli::Environment::inherit().insert("APP_DIR", env!("CARGO_MANIFEST_DIR"));

    assert_cli::Assert::example("01_menu")
        .with_env(&environment)
        .with_args(&["--timeout", "0"])
        .unwrap();
}
