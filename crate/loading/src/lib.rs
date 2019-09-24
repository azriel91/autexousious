#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a wrapper `State` around asset loading.

pub use crate::{
    loading_bundle::LoadingBundle,
    loading_state::LoadingState,
    system::{AssetDiscoverySystem, AssetLoadingSystem},
};

mod loading_bundle;
mod loading_state;
mod system;
