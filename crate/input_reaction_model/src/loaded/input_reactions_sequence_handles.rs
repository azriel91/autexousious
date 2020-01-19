use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::{input_reaction::InputReaction, InputReactionsSequenceHandle};

/// Sequence of `InputReactionsSequenceHandle`s.
#[sequence_component_data(InputReactionsSequenceHandle<InputReaction>)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct InputReactionsSequenceHandles;

impl<'s> ItemComponent<'s> for InputReactionsSequenceHandles {
    type SystemData = ();
}
