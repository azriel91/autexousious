use amethyst::ecs::prelude::*;
use game_input::ControllerId;

/// Stores the controller ID for an entity.
#[derive(Debug, new)]
pub struct CharacterEntityControl {
    /// ID of the controller that controls the character entity.
    pub controller_id: ControllerId,
}

// `DenseVecStorage` because presumably a fair number the entities on screen will be controlled by
// `Controller`s, human or AI,
impl Component for CharacterEntityControl {
    type Storage = DenseVecStorage<Self>;
}
