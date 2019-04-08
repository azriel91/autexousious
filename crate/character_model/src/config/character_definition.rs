use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_new::new;
use object_model::config::{GameObjectDefinition, ObjectDefinition};
use serde::{Deserialize, Serialize};

use crate::config::CharacterSequence;

/// Contains all of the sequences for an `Object`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct CharacterDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<CharacterSequence>,
}

impl Asset for CharacterDefinition {
    const NAME: &'static str = "object_model::config::CharacterDefinition";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<CharacterDefinition> for Result<ProcessingState<CharacterDefinition>, Error> {
    fn from(
        character_definition: CharacterDefinition,
    ) -> Result<ProcessingState<CharacterDefinition>, Error> {
        Ok(ProcessingState::Loaded(character_definition))
    }
}

impl GameObjectDefinition for CharacterDefinition {
    type GameObjectSequence = CharacterSequence;

    fn object_definition(&self) -> &ObjectDefinition<Self::GameObjectSequence> {
        &self.object_definition
    }
}
