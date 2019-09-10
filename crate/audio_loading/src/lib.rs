#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes audio configuration into the loaded audio model.

pub use crate::{audio_loader::AudioLoader, audio_loading_bundle::AudioLoadingBundle};

mod audio_loader;
mod audio_loading_bundle;
