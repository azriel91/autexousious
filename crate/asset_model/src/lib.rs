#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to reference asset configuration and entities.
//!
//! This crate contains the types necessary to reference assets configuration
//! from the file system. It does not contain the types that represent actual
//! configuration. Those are provided by the respective configuration crates.
//!
//! For example, this crate contains the [`AssetIndex`][asset_index] type, which
//! stores where object configuration is, but does not contain `ObjectType` or
//! types for the various object types.
//!
//! This crate also does not provide the logic to discover the configuration on
//! disk. That is provided by the `asset_loading` crate.
//!
//! [asset_index]: config/struct.AssetIndex.html

pub use crate::item_component::ItemComponent;

pub mod config;
pub mod loaded;
pub mod play;

mod item_component;
