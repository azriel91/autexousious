extern crate assert_cli;

#[test]
fn read_and_exit() {
    assert_cli::Assert::example("01_read_and_exit")
        .stdin("exit\n")
        .unwrap();
}

#[test]
fn read_and_exit_timeout() {
    assert_cli::Assert::example("01_read_and_exit")
        .with_args(&["-t", "0"])
        .stdin("abc\n")
        .unwrap();
}
