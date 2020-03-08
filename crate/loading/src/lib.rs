#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    loading_bundle::LoadingBundle,
    loading_state::LoadingState,
    system::{
        AssetDefinitionLoader, AssetDefinitionLoadingSystem, AssetDiscoverySystem,
        AssetDiscoverySystemData, AssetIdMapper, AssetIdMappingSystem, AssetPartLoader,
        AssetPartLoadingCoordinatorSystem, AssetPartLoadingCoordinatorSystemData,
        AssetPartLoadingSystem, AssetSequenceComponentLoader,
        AssetSequenceComponentLoaderUiCharacterSelection, AssetSequenceComponentLoaderUiComponents,
        AssetSequenceComponentLoaderUiControlSettings, AssetSequenceComponentLoaderUiForm,
        AssetSequenceComponentLoaderUiMapSelection, AssetSequenceComponentLoaderUiMenu,
        AssetSequenceComponentLoaderUiSessionLobby, AssetSequenceComponentLoadingSystem,
        AssetSpritesDefinitionLoader, AssetSpritesDefinitionLoadingSystem, AssetTextureLoader,
        AssetTextureLoadingSystem,
    },
    system_data::{
        AssetLoadingResources, DefinitionLoadingResources, DefinitionLoadingResourcesRead,
        IdMappingResources, IdMappingResourcesRead, SequenceComponentLoadingResources,
        SequenceComponentLoadingResourcesRead, SpritesDefinitionLoadingResources,
        SpritesDefinitionLoadingResourcesRead, TextureLoadingResources,
        TextureLoadingResourcesRead,
    },
};

mod loading_bundle;
mod loading_state;
mod system;
mod system_data;
