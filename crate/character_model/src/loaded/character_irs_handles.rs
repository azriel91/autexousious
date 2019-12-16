use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::CharacterIrsHandle;

/// Vector of `CharacterIrsHandle`s.
#[sequence_component_data(CharacterIrsHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CharacterIrsHandles;

impl<'s> ItemComponent<'s> for CharacterIrsHandles {
    type SystemData = ();
}
