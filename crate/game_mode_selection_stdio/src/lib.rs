#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Extension to enable `game_mode_selection` to be controlled by stdio.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
extern crate application_event;
extern crate application_menu;
#[cfg(test)]
extern crate assets_test;
extern crate game_mode_selection_model;
#[macro_use]
extern crate derive_new;
extern crate stdio_spi;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use game_mode_selection_event_args::GameModeSelectionEventArgs;
pub use game_mode_selection_event_stdin_mapper::GameModeSelectionEventStdinMapper;
pub use game_mode_selection_stdio_bundle::GameModeSelectionStdioBundle;

mod game_mode_selection_event_args;
mod game_mode_selection_event_stdin_mapper;
mod game_mode_selection_stdio_bundle;
