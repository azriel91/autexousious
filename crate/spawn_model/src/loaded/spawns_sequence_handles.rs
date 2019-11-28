use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SpawnsSequenceHandle;

/// Sequence of `SpawnsSequenceHandle`s.
#[sequence_component_data(SpawnsSequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct SpawnsSequenceHandles;

impl<'s> ItemComponent<'s> for SpawnsSequenceHandles {
    type SystemData = ();
}
