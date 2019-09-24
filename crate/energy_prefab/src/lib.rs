#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the prefab types and processing logic for energys.

pub use crate::{
    energy_entity_augmenter::EnergyEntityAugmenter, system_data::EnergyComponentStorages,
};

mod energy_entity_augmenter;
mod system_data;
