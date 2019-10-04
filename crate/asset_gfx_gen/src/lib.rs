#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Functions to generate textures and sprite sheets.

pub use crate::{
    colour_sprite_sheet_gen::ColourSpriteSheetGen,
    colour_sprite_sheet_gen_data::ColourSpriteSheetGenData,
    colour_sprite_sheet_params::ColourSpriteSheetParams, sprite_gen_params::SpriteGenParams,
    sprite_sheet_gen::SpriteSheetGen,
};

mod colour_sprite_sheet_gen;
mod colour_sprite_sheet_gen_data;
mod colour_sprite_sheet_params;
mod sprite_gen_params;
mod sprite_sheet_gen;
