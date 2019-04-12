use std::collections::HashMap;

use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_new::new;
use object_model::game_object;
use typename_derive::TypeName;

use crate::{
    config::{CharacterDefinition, CharacterSequence, CharacterSequenceId},
    loaded::CharacterControlTransitionsSequenceHandle,
};

/// Represents an in-game character that has been loaded.
///
/// Each of these fields should be a component that is attached to the character entity.
#[game_object(CharacterSequenceId, definition = CharacterDefinition)]
#[derive(Clone, Debug, PartialEq, TypeName, new)]
pub struct Character {
    /// Handles of `ControlTransitions`es sequences that this character uses, keyed by sequence ID.
    pub control_transitions_sequence_handles:
        HashMap<CharacterSequenceId, CharacterControlTransitionsSequenceHandle>,
}

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
