#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! System that integrates with standard I/O so that the application can be controlled headlessly.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate application_event;
extern crate application_input;
#[macro_use]
extern crate derive_new;
extern crate console;
extern crate fern;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate shell_words;
extern crate stdio_spi;
extern crate structopt;
extern crate strum;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub(crate) use io_app_event_utils::IoAppEventUtils;
pub use stdin_system::StdinSystem;
pub use stdio_view_bundle::StdioViewBundle;

mod io_app_event_utils;
pub(crate) mod reader;
mod stdin_system;
mod stdio_view_bundle;
