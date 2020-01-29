#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during game play.

pub use crate::{
    control_settings_entity::ControlSettingsEntity, control_settings_event::ControlSettingsEvent,
};

mod control_settings_entity;
mod control_settings_event;

pub mod config;
pub mod loaded;
