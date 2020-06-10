#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types used by asset loading SPIs.

pub use crate::system_data::{
    AssetLoadingResources, DefinitionLoadingResources, DefinitionLoadingResourcesRead,
    IdMappingResources, IdMappingResourcesRead, SequenceComponentLoadingResources,
    SequenceComponentLoadingResourcesRead, SpritesDefinitionLoadingResources,
    SpritesDefinitionLoadingResourcesRead, TextureLoadingResources, TextureLoadingResourcesRead,
};

mod system_data;
