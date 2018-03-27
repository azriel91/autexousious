extern crate assert_cli;

#[test]
fn start_and_exit() {
    assert_cli::Assert::command(&main_binary_command())
        .stdin("exit\n")
        .unwrap();
}

// Needed while <https://github.com/assert-rs/assert_cli/issues/86> is not in `assert_cli`
fn main_binary_command() -> Vec<&'static str> {
    let mut command = vec!["cargo", "run"];
    if !cfg!(debug_assertions) {
        command.push("--release");
    }
    command.push("--");
    command
}
