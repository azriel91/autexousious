#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes sprite configuration into the loaded sprite model.

pub use crate::{
    sprite_loader::SpriteLoader, sprite_loading_bundle::SpriteLoadingBundle,
    sprite_sheet_loader::SpriteSheetLoader, sprite_sheet_mapper::SpriteSheetMapper,
    texture_loader::TextureLoader,
};

mod sprite_loader;
mod sprite_loading_bundle;
mod sprite_sheet_loader;
mod sprite_sheet_mapper;
mod texture_loader;
