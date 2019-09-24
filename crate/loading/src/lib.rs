#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    loading_bundle::LoadingBundle,
    loading_state::LoadingState,
    map_asset_handles::MapAssetHandles,
    map_loading_status::MapLoadingStatus,
    object_asset_handles::ObjectAssetHandles,
    system::{AssetDiscoverySystem, AssetLoadingSystem},
};

mod loading_bundle;
mod loading_state;
mod map_asset_handles;
mod map_loading_status;
mod object_asset_handles;
mod system;
