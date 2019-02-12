#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    asset_loader::AssetLoader, loading_state::LoadingState,
    object_asset_handles::ObjectAssetHandles, system::ObjectAssetLoadingSystem,
};

mod asset_loader;
mod loading_state;
mod object_asset_handles;
mod system;
