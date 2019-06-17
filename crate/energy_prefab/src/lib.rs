#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the prefab types and processing logic for energys.

pub use crate::{
    energy_entity_augmenter::EnergyEntityAugmenter, energy_prefab::EnergyPrefab,
    energy_prefab_bundle::EnergyPrefabBundle, energy_prefab_handle::EnergyPrefabHandle,
    system_data::EnergyComponentStorages,
};

mod energy_entity_augmenter;
mod energy_prefab;
mod energy_prefab_bundle;
mod energy_prefab_handle;
mod system_data;
