#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes energy configuration into the loaded energy model.

pub use crate::{
    energy_loader::EnergyLoader,
    energy_loading_bundle::{EnergyLoadingBundle, ENERGY_PROCESSOR},
    energy_loading_status::EnergyLoadingStatus,
};

mod energy_loader;
mod energy_loading_bundle;
mod energy_loading_status;
