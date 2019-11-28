use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::CharacterCtsHandle;

/// Vector of `CharacterCtsHandle`s.
#[sequence_component_data(CharacterCtsHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CharacterCtsHandles;

impl<'s> ItemComponent<'s> for CharacterCtsHandles {
    type SystemData = ();
}
