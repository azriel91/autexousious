use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SequenceEndTransition;

/// Sequence transition upon sequence end.
#[sequence_component_data(SequenceEndTransition)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct SequenceEndTransitions;

/// `SequenceEndTransitionsSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceEndTransitionsSystemData<'s> {
    /// `SequenceEndTransition` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: WriteStorage<'s, SequenceEndTransition>,
}

impl<'s> ItemComponent<'s> for SequenceEndTransitions {
    type SystemData = SequenceEndTransitionsSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SequenceEndTransitionsSystemData {
            sequence_end_transitions,
        } = system_data;

        sequence_end_transitions
            .insert(entity, SequenceEndTransition::default())
            .expect("Failed to insert `SequenceEndTransition` component.");
    }
}
