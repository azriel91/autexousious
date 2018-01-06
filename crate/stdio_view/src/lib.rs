#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! System that integrates with standard I/O so that the application can be controlled headlessly.

extern crate amethyst;
extern crate application_input;
extern crate console;

pub use stdin_system::StdinSystem;

pub(crate) mod reader;
mod stdin_system;
