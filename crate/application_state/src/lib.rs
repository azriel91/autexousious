#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `AutexState` trait to simplify implementing `amethyst::State`.

extern crate amethyst;
extern crate application_event;

pub use autex_state::AutexState;

mod autex_state;
