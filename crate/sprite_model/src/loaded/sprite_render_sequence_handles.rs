use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    renderer::Transparent,
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SpriteRenderSequenceHandle;

/// Sequence of `SpriteRenderSequenceHandle`s.
#[sequence_component_data(SpriteRenderSequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct SpriteRenderSequenceHandles;

/// `SpriteRenderSequenceHandlesSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpriteRenderSequenceHandlesSystemData<'s> {
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
}

impl<'s> ItemComponent<'s> for SpriteRenderSequenceHandles {
    type SystemData = SpriteRenderSequenceHandlesSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SpriteRenderSequenceHandlesSystemData { transparents } = system_data;

        if !transparents.contains(entity) {
            transparents
                .insert(entity, Transparent)
                .expect("Failed to insert `Transparent` component.");
        }
    }
}
