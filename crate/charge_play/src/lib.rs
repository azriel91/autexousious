#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for game objects used during game play.

pub use crate::system::{
    ChargeIncrementSystem, ChargeInitializeDelaySystem, ChargeInitializeDetectionSystem,
    ChargeRetentionSystem, ChargeUsageSystem, CHARGE_DELAY_DEFAULT,
};

mod system;
