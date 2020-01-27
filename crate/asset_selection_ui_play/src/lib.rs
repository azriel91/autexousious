#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for asset selection UI used at runtime.

pub use crate::system::{
    ApwPreviewSpawnSystem, ApwPreviewSpawnSystemCharacter, ApwPreviewSpawnSystemMap,
    AssetSelectionSfxSystem, AssetSelectionSfxSystemData, AswPortraitUpdateSystem,
};

mod system;
