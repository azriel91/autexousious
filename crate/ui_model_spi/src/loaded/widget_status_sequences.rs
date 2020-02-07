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

use crate::config::WidgetStatus;

/// Sequence to use when transitioning to a particular `WidgetStatus`.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct WidgetStatusSequences(pub IndexMap<WidgetStatus, SequenceId>);

/// `WidgetStatusSequencesSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct WidgetStatusSequencesSystemData<'s> {
    /// `WidgetStatusSequences` components.
    #[derivative(Debug = "ignore")]
    pub widget_status_sequenceses: WriteStorage<'s, WidgetStatusSequences>,
}

impl<'s> ItemComponent<'s> for WidgetStatusSequences {
    type SystemData = WidgetStatusSequencesSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let WidgetStatusSequencesSystemData {
            widget_status_sequenceses,
        } = system_data;

        if widget_status_sequenceses.get(entity).is_none() {
            widget_status_sequenceses
                .insert(entity, self.clone()) // TODO: Make self `Copy`?
                .expect("Failed to insert `WidgetStatusSequences` component.");
        }
    }
}

impl FromIterator<(WidgetStatus, SequenceId)> for WidgetStatusSequences {
    fn from_iter<T: IntoIterator<Item = (WidgetStatus, SequenceId)>>(
        iter: T,
    ) -> WidgetStatusSequences {
        let widget_status_sequences = IndexMap::from_iter(iter);
        WidgetStatusSequences(widget_status_sequences)
    }
}
