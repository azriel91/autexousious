use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use sequence_model_derive::sequence_component_data;

use crate::{
    loaded::WaitSequenceHandle,
    play::{FrameIndexClock, FrameWaitClock},
};

/// Sequence of `WaitSequenceHandle`s.
#[sequence_component_data(WaitSequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct WaitSequenceHandles;

/// `WaitSequenceHandlesSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct WaitSequenceHandlesSystemData<'s> {
    /// `FrameWaitClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
    /// `FrameIndexClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
}

impl<'s> ItemComponent<'s> for WaitSequenceHandles {
    type SystemData = WaitSequenceHandlesSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let WaitSequenceHandlesSystemData {
            frame_wait_clocks,
            frame_index_clocks,
        } = system_data;

        if !frame_wait_clocks.contains(entity) {
            frame_wait_clocks
                .insert(entity, FrameWaitClock::new(1))
                .expect("Failed to insert `FrameWaitClock` component.");
        }
        if !frame_index_clocks.contains(entity) {
            frame_index_clocks
                .insert(entity, FrameIndexClock::new(1))
                .expect("Failed to insert `FrameIndexClock` component.");
        }
    }
}
