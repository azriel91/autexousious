extern crate assert_cli;

#[test]
fn example_01_draw_text() {
    assert_cli::Assert::command(&["cargo", "run", "--example", "01_draw_text", "--"])
        .with_args(&["--no-run"])
        .unwrap();
}
