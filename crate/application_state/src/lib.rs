#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `AutexState` trait to simplify implementing `amethyst::State`.

extern crate amethyst;
extern crate amethyst_utils;
extern crate application_event;
#[cfg(test)]
extern crate character_selection_model;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
#[cfg(test)]
extern crate rayon;

pub use app_state::{AppState, AppStateBuilder};
pub use autex_state::AutexState;

mod app_state;
mod autex_state;
