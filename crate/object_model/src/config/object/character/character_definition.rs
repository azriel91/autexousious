use amethyst::assets::{Asset, Handle};
use amethyst::ecs::prelude::*;

use config::object::character::SequenceId;
use config::object::ObjectDefinition;

/// Contains all of the sequences for an `Object`.
#[derive(Clone, Constructor, Debug, Deserialize, PartialEq)]
pub struct CharacterDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<SequenceId>,
}

impl Asset for CharacterDefinition {
    const NAME: &'static str = "object_model::config::CharacterDefinition";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}
