use std::iter::FromIterator;

use bimap::BiMap;
use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::{config, loaded::SequenceId};

/// Mappings from sequence name to ID, and ID to name.
///
/// This is essentially a wrapper around `BiMap`, with the `name()` and `id()` methods.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    /// Bi-directional mapping from sequence name to id.
    #[new(default)]
    pub sequence_name_to_id: BiMap<SeqName, SequenceId>,
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
    pub fn name(&self, sequence_id: SequenceId) -> Option<&SeqName> {
        self.sequence_name_to_id.get_by_right(&sequence_id)
    }

    /// Returns the sequence name for the given ID.
    pub fn id(&self, sequence_name: &SeqName) -> Option<&SequenceId> {
        self.sequence_name_to_id.get_by_left(sequence_name)
    }
}

impl<SeqName> FromIterator<(SeqName, SequenceId)> for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    fn from_iter<T: IntoIterator<Item = (SeqName, SequenceId)>>(
        iter: T,
    ) -> SequenceIdMappings<SeqName> {
        let sequence_name_to_id = BiMap::from_iter(iter);
        SequenceIdMappings {
            sequence_name_to_id,
        }
    }
}
