#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! System that integrates with standard I/O so that the application can be controlled headlessly.

extern crate console;
extern crate dialoguer;
extern crate shred;
extern crate specs;

use std::io::Write;

use console::Term;
use console::style;
use dialoguer::Input;

use shred::Resources;
use specs::RunNow;

/// Rendering system.
#[derive(Debug)]
pub struct StdinSystem;

impl<'a> RunNow<'a> for StdinSystem {
    fn run_now(&mut self, _res: &'a Resources) {
        loop {
            let mut term = Term::stdout();

            let prompt = format!("{}", style("Enter name").blue().bold());
            let input = Input::new(&prompt).interact_on(&term).unwrap();

            writeln!(term, "Hello {}!", input).unwrap();
        }
    }
}
