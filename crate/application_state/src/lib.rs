#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `AppState` trait to simplify implementing `amethyst::State`.

extern crate amethyst;
extern crate application_event;
#[cfg(test)]
extern crate character_selection;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;

pub use app_state::AppState;
pub use state_proxy::StateProxy;

mod app_state;
mod state_proxy;
