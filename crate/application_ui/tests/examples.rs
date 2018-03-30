extern crate assert_cli;

#[test]
fn example_01_draw_text() {
    assert_cli::Assert::command(&example_command("01_draw_text"))
        .with_args(&["--no-run"])
        .unwrap();
}

// Needed while <https://github.com/assert-rs/assert_cli/issues/86> is not in `assert_cli`
fn example_command(example: &'static str) -> Vec<&str> {
    let mut command = vec!["cargo", "run"];
    if !cfg!(debug_assertions) {
        command.push("--release");
    }
    command.extend(&["--example", example, "--"]);
    command
}
