use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;

/// `SequenceId`s of map selection portraits.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct MswPortraits {
    /// Used when map selection is "Random".
    pub random: SequenceId,
    /// Used when map selection is a map.
    pub select: SequenceId,
}

/// `MswPortraitsSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MswPortraitsSystemData<'s> {
    /// `MswPortraits` components.
    #[derivative(Debug = "ignore")]
    pub msw_portraitses: WriteStorage<'s, MswPortraits>,
}

impl<'s> ItemComponent<'s> for MswPortraits {
    type SystemData = MswPortraitsSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let MswPortraitsSystemData { msw_portraitses } = system_data;

        if msw_portraitses.get(entity).is_none() {
            msw_portraitses
                .insert(entity, *self)
                .expect("Failed to insert `MswPortraits` component.");
        }
    }
}
