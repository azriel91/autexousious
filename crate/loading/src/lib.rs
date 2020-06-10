#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    loading_bundle::LoadingBundle,
    loading_state::LoadingState,
    system::{
        AssetDefinitionLoader, AssetDefinitionLoadingSystem, AssetDiscoverySystem,
        AssetDiscoverySystemData, AssetIdMapper, AssetIdMappingSystem, AssetPartLoader,
        AssetPartLoadingCoordinatorSystem, AssetPartLoadingCoordinatorSystemData,
        AssetPartLoadingSystem, AssetSequenceComponentLoader, AssetSequenceComponentLoadingSystem,
        AssetSpritesDefinitionLoader, AssetSpritesDefinitionLoadingSystem, AssetTextureLoader,
        AssetTextureLoadingSystem, UiAsclCharacterSelection, UiAsclComponents,
        UiAsclControlSettings, UiAsclForm, UiAsclMapSelection, UiAsclMenu, UiAsclSessionLobby,
    },
};

mod loading_bundle;
mod loading_state;
mod system;
