use amethyst::ecs::{prelude::*, storage::HashMapStorage};
use game_input::ControllerId;

/// Stores the human controller ID for an entity.
///
/// We use a `HashMapStorage` because there wouldn't be that many entities that are controlled by
/// human `Controller`s. We will use a different `Component` for AI controllers.
#[derive(Debug, new)]
pub struct CharacterEntityControl {
    /// ID of the controller that controls the character entity.
    pub controller_id: ControllerId,
}

impl Component for CharacterEntityControl {
    type Storage = HashMapStorage<Self>;
}
