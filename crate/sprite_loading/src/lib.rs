#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes sprite configuration into the loaded sprite model.

pub use crate::{
    scale_sequence_handles_loader::ScaleSequenceHandlesLoader,
    scale_sequence_loader::ScaleSequenceLoader, sprite_loader::SpriteLoader,
    sprite_loading_bundle::SpriteLoadingBundle,
    sprite_render_sequence_handles_loader::SpriteRenderSequenceHandlesLoader,
    sprite_render_sequence_loader::SpriteRenderSequenceLoader,
    sprite_sheet_loader::SpriteSheetLoader, sprite_sheet_mapper::SpriteSheetMapper,
    texture_loader::TextureLoader, tint_sequence_handles_loader::TintSequenceHandlesLoader,
    tint_sequence_loader::TintSequenceLoader,
};

mod scale_sequence_handles_loader;
mod scale_sequence_loader;
mod sprite_loader;
mod sprite_loading_bundle;
mod sprite_render_sequence_handles_loader;
mod sprite_render_sequence_loader;
mod sprite_sheet_loader;
mod sprite_sheet_mapper;
mod texture_loader;
mod tint_sequence_handles_loader;
mod tint_sequence_loader;
