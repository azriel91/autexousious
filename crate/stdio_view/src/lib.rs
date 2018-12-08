#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! System that integrates with standard I/O so that the application can be controlled headlessly.

#[macro_use]
extern crate log;
use shell_words;

use typename;
#[macro_use]
extern crate typename_derive;

pub(crate) use crate::io_app_event_utils::IoAppEventUtils;
pub use crate::{stdin_system::StdinSystem, stdio_view_bundle::StdioViewBundle};

mod io_app_event_utils;
pub(crate) mod reader;
mod stdin_system;
mod stdio_view_bundle;
