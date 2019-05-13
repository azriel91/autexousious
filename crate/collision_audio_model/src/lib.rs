#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Logical IDs of audio used during collision.

pub use crate::collision_audio_loading_status::CollisionAudioLoadingStatus;

pub mod config;
pub mod loaded;

mod collision_audio_loading_status;
