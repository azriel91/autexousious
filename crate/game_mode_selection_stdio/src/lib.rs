#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Extension to enable `game_mode_selection` to be controlled by stdio.







#[macro_use]
extern crate derive_new;

use structopt;
#[macro_use]
extern crate structopt_derive;
use typename;
#[macro_use]
extern crate typename_derive;

pub use crate::game_mode_selection_event_args::GameModeSelectionEventArgs;
pub use crate::game_mode_selection_event_stdin_mapper::GameModeSelectionEventStdinMapper;
pub use crate::game_mode_selection_stdio_bundle::GameModeSelectionStdioBundle;

mod game_mode_selection_event_args;
mod game_mode_selection_event_stdin_mapper;
mod game_mode_selection_stdio_bundle;
