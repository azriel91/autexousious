#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `AppState` trait to simplify implementing `amethyst::State`.

extern crate amethyst;
extern crate application_event;

pub use app_state::AppState;

mod app_state;
