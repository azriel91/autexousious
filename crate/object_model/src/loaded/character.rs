use amethyst::{
    assets::{Asset, Error, Handle, ProcessingState},
    ecs::prelude::*,
};
use derivative::Derivative;
use derive_new::new;
use object_model_derive::GameObject;

use crate::{
    config::object::CharacterSequenceId,
    loaded::{GameObject, ObjectHandle, SequenceEndTransitions},
};

/// Represents an in-game character that has been loaded.
///
/// Each of these fields should be a component that is attached to the character entity.
#[derive(Clone, Derivative, GameObject, PartialEq, new)]
#[derivative(Debug)]
pub struct Character {
    /// Handle to loaded object data.
    pub object_handle: ObjectHandle<CharacterSequenceId>,
    /// Component sequence transitions when a sequence ends.
    pub sequence_end_transitions: SequenceEndTransitions<CharacterSequenceId>,
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
