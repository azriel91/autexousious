#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Logical IDs of audio used in UIs.

pub use crate::ui_audio_loading_status::UiAudioLoadingStatus;

pub mod config;
pub mod loaded;

mod ui_audio_loading_status;
