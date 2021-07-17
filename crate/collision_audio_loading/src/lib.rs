#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes collision audio configuration into the loaded collision audio
//! model.

pub use crate::{
    collision_audio_loading_bundle::CollisionAudioLoadingBundle,
    system::CollisionAudioLoadingSystem,
};

mod collision_audio_loading_bundle;
mod system;
