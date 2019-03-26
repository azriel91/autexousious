#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! System that integrates with standard I/O so that the application can be controlled headlessly.

pub(crate) use crate::{
    io_app_event_utils::IoAppEventUtils, stdin_command_barrier::StdinCommandBarrier,
};
pub use crate::{stdio_view_bundle::StdioViewBundle, system::StdinSystem};

mod io_app_event_utils;
pub(crate) mod reader;
mod stdin_command_barrier;
mod stdio_view_bundle;
mod system;
