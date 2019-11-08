//! User defined configuration types for sprites.

pub use self::{
    sprite_frame::SpriteFrame, sprite_offset::SpriteOffset, sprite_position::SpritePosition,
    sprite_ref::SpriteRef, sprite_sequence::SpriteSequence,
    sprite_sequence_name::SpriteSequenceName, sprite_sheet_definition::SpriteSheetDefinition,
    sprites_definition::SpritesDefinition, tint::Tint,
};

mod sprite_frame;
mod sprite_offset;
mod sprite_position;
mod sprite_ref;
mod sprite_sequence;
mod sprite_sequence_name;
mod sprite_sheet_definition;
mod sprites_definition;
mod tint;
