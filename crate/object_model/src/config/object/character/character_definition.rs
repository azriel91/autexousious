use amethyst::{
    assets::{Asset, Handle},
    ecs::prelude::*,
};

use config::object::{CharacterSequenceId, ObjectDefinition};

/// Contains all of the sequences for an `Object`.
#[derive(Clone, Constructor, Debug, Deserialize, PartialEq)]
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
