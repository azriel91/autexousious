#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Provides logic used during game play.

pub use crate::system::{
    HitDetectionSystem, HitRepeatTrackersAugmentSystem, HitRepeatTrackersTickerSystem,
};

mod system;
