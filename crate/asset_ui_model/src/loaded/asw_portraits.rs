use std::iter::FromIterator;

use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use indexmap::IndexMap;
use sequence_model::loaded::SequenceId;

use crate::config::AswPortraitName;

/// Portraits available to an `AssetSelectionWidget`.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct AswPortraits(pub IndexMap<AswPortraitName, SequenceId>);

/// `AswPortraitsSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AswPortraitsSystemData<'s> {
    /// `AswPortraits` components.
    #[derivative(Debug = "ignore")]
    pub asw_portraitses: WriteStorage<'s, AswPortraits>,
}

impl<'s> ItemComponent<'s> for AswPortraits {
    type SystemData = AswPortraitsSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AswPortraitsSystemData { asw_portraitses } = system_data;

        if asw_portraitses.get(entity).is_none() {
            asw_portraitses
                .insert(entity, self.clone()) // TODO: Make self `Copy`?
                .expect("Failed to insert `AswPortraits` component.");
        }
    }
}

impl FromIterator<(AswPortraitName, SequenceId)> for AswPortraits {
    fn from_iter<T: IntoIterator<Item = (AswPortraitName, SequenceId)>>(iter: T) -> AswPortraits {
        let asw_portraits = IndexMap::from_iter(iter);
        AswPortraits(asw_portraits)
    }
}
