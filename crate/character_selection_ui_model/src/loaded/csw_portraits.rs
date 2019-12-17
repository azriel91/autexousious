use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_new::new;
use sequence_model::loaded::SequenceId;

/// `SequenceId`s of character selection portraits.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
pub struct CswPortraits {
    /// Used when the widget is inactive.
    pub join: SequenceId,
    /// Used when character selection is "Random".
    pub random: SequenceId,
}

/// `CswPortraitsSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswPortraitsSystemData<'s> {
    /// `CswPortraits` components.
    #[derivative(Debug = "ignore")]
    pub csw_portraitses: WriteStorage<'s, CswPortraits>,
}

impl<'s> ItemComponent<'s> for CswPortraits {
    type SystemData = CswPortraitsSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let CswPortraitsSystemData { csw_portraitses } = system_data;

        if csw_portraitses.get(entity).is_none() {
            csw_portraitses
                .insert(entity, *self)
                .expect("Failed to insert `CswPortraits` component.");
        }
    }
}
