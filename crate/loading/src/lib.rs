#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides a wrapper `State` around resource loading.

extern crate amethyst;
extern crate amethyst_animation;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_error_chain;
extern crate error_chain;
extern crate game_config;
#[macro_use]
extern crate log;
extern crate object_config;
extern crate toml;

pub use state::State;

pub mod texture;

mod error;
mod object_loader;
mod sprite;
mod state;
