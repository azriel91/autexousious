use amethyst::ecs::{prelude::*, storage::DenseVecStorage};

use entity::RunCounter;

/// Character-specific status for character entities.
///
/// We use a `DenseVecStorage` because all character entities, but not all entities will have this.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct CharacterStatus {
    /// Tracks state used to determine when a character should run.
    pub run_counter: RunCounter,
}

impl Component for CharacterStatus {
    type Storage = DenseVecStorage<Self>;
}
