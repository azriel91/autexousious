use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::ScaleSequenceHandle;

/// Sequence of `ScaleSequenceHandle`s.
#[sequence_component_data(ScaleSequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ScaleSequenceHandles;

impl<'s> ItemComponent<'s> for ScaleSequenceHandles {
    type SystemData = ();
}
