extern crate assert_cli;

#[test]
fn start_and_exit() {
    assert_cli::Assert::main_binary()
        // Even though XVFB starts, creation of the window still fails with NoAvailablePixelFormat.
        // Only some jobs will report this depending on whether stderr is captured before the
        // test execution ends.
        //
        // In such a case, stdin does not get read, so we have no choice but to disable creation of
        // the window.
        .with_args(&["--headless"])
        .stdin("exit\n")
        .unwrap();
}
