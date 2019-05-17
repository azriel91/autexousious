#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes ui audio configuration into the loaded ui audio model.

pub use crate::{system::UiAudioLoadingSystem, ui_audio_loading_bundle::UiAudioLoadingBundle};

mod system;
mod ui_audio_loading_bundle;
