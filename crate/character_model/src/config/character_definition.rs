use amethyst::{
    assets::{Asset, Handle},
    ecs::storage::VecStorage,
};
use derive_new::new;
use object_model::config::{object::ObjectDefinition, GameObjectDefinition};
use serde::{Deserialize, Serialize};

use crate::config::CharacterSequenceId;

/// Contains all of the sequences for an `Object`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct CharacterDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<CharacterSequenceId>,
}

impl Asset for CharacterDefinition {
    const NAME: &'static str = "object_model::config::CharacterDefinition";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl GameObjectDefinition for CharacterDefinition {
    type SequenceId = CharacterSequenceId;

    fn object_definition(&self) -> &ObjectDefinition<Self::SequenceId> {
        &self.object_definition
    }
}
