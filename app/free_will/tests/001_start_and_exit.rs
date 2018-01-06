extern crate assert_cli;

#[test]
fn start_and_exit() {
    assert_cli::Assert::main_binary()
        .with_args(&["--headless"])
        .stdin("exit\n")
        .unwrap();
}
