#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes kinematic configuration into the loaded kinematic model.

pub use crate::{
    kinematic_loading_bundle::KinematicLoadingBundle, position_inits_loader::PositionInitsLoader,
};

mod kinematic_loading_bundle;
mod position_inits_loader;
