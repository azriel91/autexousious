#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides a wrapper `State` around asset loading.

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate log;

pub use crate::{asset_loader::AssetLoader, loading_state::LoadingState};

mod asset_loader;
mod loading_state;
