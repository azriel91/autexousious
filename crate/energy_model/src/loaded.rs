//! Contains the types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::{
    asset_energy_definition_handle::AssetEnergyDefinitionHandle,
    energy::{Energy, EnergyHandle, EnergyObjectWrapper},
};

mod asset_energy_definition_handle;
mod energy;
