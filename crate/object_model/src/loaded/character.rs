use amethyst::assets::{Asset, Handle};
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
