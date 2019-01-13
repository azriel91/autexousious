use amethyst::{
    assets::{Asset, Error, Handle, ProcessingState},
    ecs::storage::VecStorage,
};
use derive_new::new;
use object_model::game_object;

use crate::config::CharacterSequenceId;

/// Represents an in-game character that has been loaded.
///
/// Each of these fields should be a component that is attached to the character entity.
#[game_object(CharacterSequenceId)]
#[derive(Clone, Debug, PartialEq, new)]
pub struct Character;

impl Asset for Character {
    const NAME: &'static str = "character_model::loaded::Character";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<Character> for Result<ProcessingState<Character>, Error> {
    fn from(character: Character) -> Result<ProcessingState<Character>, Error> {
        Ok(ProcessingState::Loaded(character))
    }
}

/// Handle to a Character
pub type CharacterHandle = Handle<Character>;
