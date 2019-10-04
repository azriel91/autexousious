//! User defined configuration types for sprites.

pub use self::{
    sprite_offset::SpriteOffset, sprite_ref::SpriteRef,
    sprite_sheet_definition::SpriteSheetDefinition, sprites_definition::SpritesDefinition,
};

mod sprite_offset;
mod sprite_ref;
mod sprite_sheet_definition;
mod sprites_definition;
