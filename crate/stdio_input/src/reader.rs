use std::{
    io::{self, Write},
    sync::mpsc::Sender,
};

use console::{style, Term};
use log::{debug, info, trace, warn};

use crate::Terminator;

/// Name of this reader, useful when naming threads.
pub const NAME: &str = concat!(module_path!(), "::StdinReader");

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
    /// Phrase that signals the application should quit.
    pub const EXIT_PHRASE: &'static str = "exit";

    // kcov-ignore-start
    /// Returns a StdinReader.
    ///
    /// # Parameters:
    ///
    /// * `system_tx`: Channel sender to `StdinSystem` for input from stdin.
    pub fn new(system_tx: Sender<String>) -> Self {
        StdinReader { system_tx }
    }

    /// Signals this reader to read from stdin.
    pub fn start(&self) {
        let mut term = Term::stdout();
        let prompt = format!("{}: ", style(">>").blue().bold());

        let mut buffer = String::new();
        loop {
            write!(term, "{}", &prompt).expect("Failed to write stdio prompt");
            match io::stdin().read_line(&mut buffer) {
                Ok(n) => {
                    if n > 0 {
                        buffer.truncate(n);
                        trace!("Input from stdin: `{:?}`.", buffer);

                        let trimmed = buffer.trim();
                        if trimmed.is_empty() {
                            continue;
                        }

                        // TODO: Terminator does some chomping, so we cannot use the returned string
                        // to attempt to read multiple commands in one line.
                        let payload = Terminator::new(trimmed.bytes()).terminate();
                        match &payload {
                            // `command_chain`: command_one args && command_two args
                            Some(Ok(command_chain)) => {
                                let should_exit = command_chain == Self::EXIT_PHRASE;

                                if let Err(command_chain) =
                                    self.system_tx.send(command_chain.to_string())
                                {
                                    warn!(
                                        "Channel sender to `StdinSystem` disconnected. Payload: \"{}\"",
                                        command_chain
                                    );
                                    break;
                                }

                                // This prevents the thread that is running this function from panicking due
                                // to accessing stdin while the application is exiting.
                                if should_exit {
                                    info!("StdinReader thread terminating.");
                                    break;
                                }
                            }
                            Some(Err(())) | None => {}
                        }
                    }
                }
                Err(e) => {
                    debug!("{}", e);
                    debug!("ErrorKind: {:?}", e.kind());
                    info!("StdinReader thread terminating.");
                    break;
                }
            }

            buffer.clear();
        }
    }
    // kcov-ignore-end
}
