use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use std::iter::FromIterator;

use bimap::BiMap;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::{
    config::{self, SequenceNameString},
    loaded::SequenceId,
};

/// Mappings from sequence name to ID, and ID to name.
///
/// This is essentially a wrapper around `BiMap`, with the `name()` and `id()` methods.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new, ItemComponent)]
#[storage(DenseVecStorage)]
pub struct SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    /// Bi-directional mapping from sequence name to id.
    #[new(default)]
    pub sequence_name_to_id: BiMap<SequenceNameString<SeqName>, SequenceId>,
}

impl<SeqName> SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    /// Returns a `SequenceIdMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        SequenceIdMappings {
            sequence_name_to_id: BiMap::with_capacity(capacity),
        }
    }

    /// Returns the sequence name for the given ID.
    pub fn name(&self, sequence_id: SequenceId) -> Option<&SequenceNameString<SeqName>> {
        self.sequence_name_to_id.get_by_right(&sequence_id)
    }

    /// Returns the sequence ID for the given sequence name string.
    pub fn id(&self, sequence_name_string: &SequenceNameString<SeqName>) -> Option<&SequenceId> {
        self.sequence_name_to_id.get_by_left(sequence_name_string)
    }

    /// Returns the sequence ID for the given `SequenceName`.
    pub fn id_by_name(&self, sequence_name: SeqName) -> Option<&SequenceId> {
        self.sequence_name_to_id
            .get_by_left(&SequenceNameString::Name(sequence_name))
    }
}

impl<SeqName> FromIterator<(SequenceNameString<SeqName>, SequenceId)>
    for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    fn from_iter<T: IntoIterator<Item = (SequenceNameString<SeqName>, SequenceId)>>(
        iter: T,
    ) -> SequenceIdMappings<SeqName> {
        let sequence_name_to_id = BiMap::from_iter(iter);
        SequenceIdMappings {
            sequence_name_to_id,
        }
    }
}

impl<'s, SeqName> FromIterator<&'s SequenceNameString<SeqName>> for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    fn from_iter<T: IntoIterator<Item = &'s SequenceNameString<SeqName>>>(
        iter: T,
    ) -> SequenceIdMappings<SeqName> {
        let iter = iter.into_iter().map(Clone::clone);

        SequenceIdMappings::from_iter(iter)
    }
}

impl<SeqName> FromIterator<SequenceNameString<SeqName>> for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    fn from_iter<T: IntoIterator<Item = SequenceNameString<SeqName>>>(
        iter: T,
    ) -> SequenceIdMappings<SeqName> {
        let iter = iter
            .into_iter()
            .enumerate()
            .map(|(index, sequence_name_string)| (sequence_name_string, SequenceId::new(index)));

        SequenceIdMappings::from_iter(iter)
    }
}
