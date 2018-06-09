use amethyst::assets::{Asset, Error, Handle, ProcessingState};
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

impl From<Character> for Result<ProcessingState<Character>, Error> {
    fn from(character: Character) -> Result<ProcessingState<Character>, Error> {
        Ok(ProcessingState::Loaded(character))
    }
}

/// Handle to a Character
pub type CharacterHandle = Handle<Character>;
