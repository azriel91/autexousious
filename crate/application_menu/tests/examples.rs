extern crate assert_cli;

#[test]
fn example_01_menu() {
    assert_cli::Assert::command(&["cargo", "run", "--example", "01_menu", "--"])
        .with_args(&["--timeout", "0"])
        .unwrap();
}
