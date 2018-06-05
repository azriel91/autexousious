use amethyst::assets::{Asset, Handle, Result};
use amethyst::ecs::prelude::*;

use config::CharacterDefinition;
use loaded::Object;

/// Represents an in-game character that has been loaded.
#[derive(Constructor, Clone, Derivative)]
#[derivative(Debug)]
pub struct Character {
    /// Common loaded object data.
    pub object: Object,
    /// Character configuration
    pub definition: CharacterDefinition,
}

impl Asset for Character {
    const NAME: &'static str = "object_model::loaded::Character";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

impl From<Character> for Result<Character> {
    fn from(character_definition: Character) -> Result<Character> {
        Ok(character_definition)
    }
}

/// Handle to a Character
pub type CharacterHandle = Handle<Character>;
