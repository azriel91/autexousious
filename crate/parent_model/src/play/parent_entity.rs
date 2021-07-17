use amethyst::ecs::{storage::DenseVecStorage, Component, Entity};
use derive_new::new;

/// Links a child entity to the parent entity.
///
/// This component should be attached to the child entity.
///
/// When a `ParentEntity` is no longer alive, the entity with this component
/// should be deleted.
///
/// **Note:** This is **not** the component attached to entities when they are
/// spawned. For that you are looking for the `spawn_model::play::SpawnParent`
/// component.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct ParentEntity(pub Entity);
