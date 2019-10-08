#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    loading_bundle::LoadingBundle,
    loading_state::LoadingState,
    system::{
        AssetDefinitionLoadingSystem, AssetDiscoverySystem, AssetIdMappingSystem,
        AssetLoadingCompleteSystem, AssetSequenceComponentLoadingSystem, AssetSpritesLoadingSystem,
        AssetTextureLoadingSystem,
    },
    system_data::{
        AssetLoadingResources, DefinitionLoadingResources, IdMappingResources,
        SequenceComponentResources, SpritesDefinitionLoadingResources, TextureLoadingResources,
    },
};

mod loading_bundle;
mod loading_state;
mod system;
mod system_data;
