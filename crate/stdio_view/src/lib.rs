#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! System that integrates with standard I/O so that the application can be controlled headlessly.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate application_input;
#[macro_use]
extern crate derive_new;
extern crate console;
extern crate fern;
#[macro_use]
extern crate log;

pub use stdin_system::StdinSystem;
pub use stdio_view_bundle::StdioViewBundle;

pub(crate) mod reader;
mod stdin_system;
mod stdio_view_bundle;
