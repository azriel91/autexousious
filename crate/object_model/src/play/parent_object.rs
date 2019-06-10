use amethyst::ecs::{storage::DenseVecStorage, Component, Entity};
use derive_new::new;
use specs_derive::Component;

/// Links a child entity to the parent object entity.
///
/// This component should be attached to the child entity.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct ParentObject {
    /// The parent entity.
    pub entity: Entity,
}
