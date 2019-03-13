#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Functions to generate textures and sprite sheets.

pub use crate::{
    colour_sprite_sheet_gen::ColourSpriteSheetGen,
    colour_sprite_sheet_gen_data::ColourSpriteSheetGenData,
    colour_sprite_sheet_params::ColourSpriteSheetParams,
};

mod colour_sprite_sheet_gen;
mod colour_sprite_sheet_gen_data;
mod colour_sprite_sheet_params;
