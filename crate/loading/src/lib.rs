#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides a wrapper `State` around resource loading.

extern crate amethyst;
extern crate application_ui;
#[macro_use]
extern crate derivative;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate map_loading;
extern crate map_model;
extern crate object_loading;
extern crate object_model;
extern crate strum;
extern crate toml;

pub use asset_loader::AssetLoader;
pub use loading_state::LoadingState;

mod asset_loader;
mod loading_state;
