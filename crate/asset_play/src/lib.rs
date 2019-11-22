#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for entities at runtime.

pub use crate::{
    asset_play_bundle::AssetPlayBundle,
    system::{ItemComponentComponentAugmentSystem, ItemIdEventSystem},
};

mod asset_play_bundle;
mod system;
