use amethyst::ecs::{storage::DenseVecStorage, Component, Entity};
use derive_new::new;

/// Links a chasing entity to the target object entity.
///
/// This component should be attached to the chasing entity.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct TargetObject {
    /// The target entity.
    pub entity: Entity,
}
