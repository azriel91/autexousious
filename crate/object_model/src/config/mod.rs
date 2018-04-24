//! Contains the types that represent the configuration on disk.

pub use self::object::{CharacterDefinition, ObjectDefinition};
pub use self::sprite::{SpriteOffset, SpriteSheetDefinition, SpritesDefinition};

pub mod object;
mod sprite;
