use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::PositionInit;
use sequence_model::loaded::SequenceId;
use serde::{Deserialize, Serialize};

/// Defines a sprite sequence to display.
#[derive(Clone, Debug, Deserialize, Component, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
#[storage(DenseVecStorage)]
pub struct UiSpriteLabel {
    /// Position of the label relative to its parent.
    pub position: PositionInit,
    /// `SequenceId` that the `UiSpriteLabel` should begin with.
    pub sequence_id: SequenceId,
}

/// `UiSpriteLabelSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiSpriteLabelSystemData<'s> {
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> ItemComponent<'s> for UiSpriteLabel {
    type SystemData = UiSpriteLabelSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiSpriteLabelSystemData { sequence_ids } = system_data;

        sequence_ids
            .insert(entity, self.sequence_id)
            .expect("Failed to insert `SequenceId` component.");
    }
}
