use amethyst::ecs::{storage::VecStorage, Component, Entity};
use derive_new::new;

/// Links a child entity to a parent entity that has a `CharacterSelection` component.
///
/// This component should be attached to the child entity.
///
/// This will allow the child entity to display a sprite sequence depending on the parent's
/// `CharacterSelection`.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct CharacterSelectionParent(Entity);
