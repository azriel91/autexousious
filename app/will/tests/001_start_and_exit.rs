extern crate assert_cli;

#[test]
fn start_and_exit() {
    // We need to inherit the environment, otherwise we won't be able to run `cargo`. See
    // <https://github.com/assert-rs/assert_cli/issues/58>
    let environment =
        assert_cli::Environment::inherit().insert("APP_DIR", env!("CARGO_MANIFEST_DIR"));

    assert_cli::Assert::main_binary()
        .with_env(&environment)
        .stdin("exit\n")
        .unwrap();
}
