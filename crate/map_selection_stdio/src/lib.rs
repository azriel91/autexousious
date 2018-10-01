#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Extension to enable `map_selection` to be controlled by stdio.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate application_event;
#[cfg(test)]
extern crate application_test_support;
#[cfg(test)]
extern crate assets_test;
extern crate map_selection_model;
#[macro_use]
extern crate derive_new;
extern crate game_model;
extern crate stdio_spi;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use map_selection_event_args::MapSelectionEventArgs;
pub use map_selection_event_stdin_mapper::MapSelectionEventStdinMapper;
pub use map_selection_stdio_bundle::MapSelectionStdioBundle;

mod map_selection_event_args;
mod map_selection_event_stdin_mapper;
mod map_selection_stdio_bundle;
