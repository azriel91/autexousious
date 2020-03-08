#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Data types representing input received from online play.

pub use crate::network_input_event::NetworkInputEvent;

pub mod play;

mod network_input_event;
