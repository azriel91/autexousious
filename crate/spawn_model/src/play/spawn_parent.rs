use amethyst::ecs::{storage::DenseVecStorage, Component, Entity};
use derive_new::new;

/// Links a spawned entity to the game object entity that spawned this.
///
/// This component should be attached to the spawned entity. In the case that
/// frame components are attached to separate child entities of a game object,
/// this should point to the main game object entity.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct SpawnParent {
    /// The parent entity.
    pub entity: Entity,
}
