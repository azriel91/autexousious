#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! System that integrates with standard I/O so that the application can be controlled headlessly.

pub(crate) use crate::ion::{StatementSplitter, StatementVariant, Terminator};
pub use crate::{
    io_app_event_utils::IoAppEventUtils,
    stdio_input_bundle::StdioInputBundle,
    system::{StdinSystem, StdinSystemData, StdinSystemDesc},
};

pub(crate) mod ion;
pub(crate) mod reader;

mod io_app_event_utils;
mod stdio_input_bundle;
mod system;
