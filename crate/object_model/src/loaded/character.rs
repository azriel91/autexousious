use amethyst::{
    assets::{Asset, Error, Handle, ProcessingState},
    ecs::prelude::*,
};

use config::{object::CharacterSequenceId, CharacterDefinition};
use loaded::ObjectHandle;

/// Represents an in-game character that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Character {
    /// Handle to loaded object data.
    pub object: ObjectHandle<CharacterSequenceId>,
    /// Character configuration.
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
