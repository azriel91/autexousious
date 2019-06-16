use asset_derive::Asset;
use derive_new::new;
use object_model::config::{GameObjectDefinition, ObjectDefinition};
use serde::{Deserialize, Serialize};

use crate::config::CharacterSequence;

/// Contains all of the sequences for a `Character`.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct CharacterDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<CharacterSequence>,
}

impl GameObjectDefinition for CharacterDefinition {
    type GameObjectSequence = CharacterSequence;

    fn object_definition(&self) -> &ObjectDefinition<Self::GameObjectSequence> {
        &self.object_definition
    }
}
