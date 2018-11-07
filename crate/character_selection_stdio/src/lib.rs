#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Extension to enable `character_selection` to be controlled by stdio.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
extern crate application_event;
#[cfg(test)]
extern crate application_test_support;
#[cfg(test)]
extern crate assets_test;
extern crate character_selection_model;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_model;
extern crate stdio_spi;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use character_selection_event_args::CharacterSelectionEventArgs;
pub use character_selection_event_stdin_mapper::CharacterSelectionEventStdinMapper;
pub use character_selection_stdio_bundle::CharacterSelectionStdioBundle;

mod character_selection_event_args;
mod character_selection_event_stdin_mapper;
mod character_selection_stdio_bundle;
