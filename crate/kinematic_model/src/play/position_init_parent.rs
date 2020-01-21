use amethyst::ecs::{storage::VecStorage, Component, Entity};
use derive_new::new;

/// Links a child entity to a parent entity with a `Position<f32>` component.
///
/// This offsets the child entity's `PositionInit` by the parent's `Position<f32>` when first
/// augmented.
///
/// This component should be attached to the child entity.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct PositionInitParent(pub Entity);
