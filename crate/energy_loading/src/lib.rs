#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes energy configuration into the loaded energy model.

pub use crate::energy_loading_bundle::{EnergyLoadingBundle, ENERGY_PROCESSOR};

mod energy_loading_bundle;
