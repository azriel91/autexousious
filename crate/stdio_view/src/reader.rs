use std::sync::mpsc::Sender;

use console::style;
use console::Term;
use dialoguer::Input;

/// Name of this reader, useful when naming threads.
pub const NAME: &'static str = concat!(module_path!(), "::StdinReader");

/// Reads lines from stdin and sends them to the [`StdinSystem`](struct.StdinSystem.html).
///
/// This should be run in a separate thread to the system as input from stdin is blocking, and the
/// system needs to be responsive to changes in the ECS `World`.
#[derive(Debug)]
pub struct StdinReader {
    /// Channel sender to the endpoint for input from stdin.
    system_tx: Sender<String>,
}

impl StdinReader {
    /// Returns a StdinReader.
    ///
    /// # Parameters:
    ///
    /// * `system_tx`: Channel sender to the System for input from stdin.
    pub fn new(system_tx: Sender<String>) -> Self {
        StdinReader { system_tx }
    }

    /// Signals this reader to read from stdin.
    pub fn start(&self) {
        let term = Term::stdout();
        let prompt = format!("{}", style(">>").blue().bold());
        let input = Input::new(&prompt);

        loop {
            let msg = input.interact_on(&term).unwrap();
            if let Err(_) = self.system_tx.send(msg) {
                // TODO: log
                break;
            }
        }
    }
}

// TODO: integration test
