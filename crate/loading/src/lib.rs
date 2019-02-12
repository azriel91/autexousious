#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    asset_loader::AssetLoader, loading_bundle::LoadingBundle, loading_state::LoadingState,
    loading_status::LoadingStatus, object_asset_handles::ObjectAssetHandles,
    system::ObjectAssetLoadingSystem,
};

mod asset_loader;
mod loading_bundle;
mod loading_state;
mod loading_status;
mod object_asset_handles;
mod system;
