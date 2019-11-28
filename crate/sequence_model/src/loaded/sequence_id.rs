use amethyst::{
    ecs::{
        storage::{FlaggedStorage, VecStorage},
        Component, Entity, World, WriteStorage,
    },
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};
use typename_derive::TypeName;

use crate::play::SequenceStatus;

/// Sequence ID of an object.
#[numeric_newtype]
#[derive(Debug, Default, Deserialize, Hash, Serialize, TypeName)]
pub struct SequenceId(pub usize);

/// Not every entity will have this, but since this is probably a `u8`, we don't need an indirection
/// table.
impl Component for SequenceId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// `SequenceIdSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceIdSystemData<'s> {
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
}

impl<'s> ItemComponent<'s> for SequenceId {
    type SystemData = SequenceIdSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let SequenceIdSystemData {
            sequence_ids,
            sequence_statuses,
        } = system_data;

        if sequence_ids.get(entity).is_none() {
            sequence_ids
                .insert(entity, *self)
                .expect("Failed to insert `SequenceId` component.");
        }
        if sequence_statuses.get(entity).is_none() {
            sequence_statuses
                .insert(entity, SequenceStatus::default())
                .expect("Failed to insert `SequenceStatus` component.");
        }
    }
}
