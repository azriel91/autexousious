use std::{collections::HashMap, iter::FromIterator};

use derive_new::new;

use crate::{config, loaded::SequenceId};

/// Mappings from sequence name to ID, and ID to name.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    /// Maps sequence ID to sequence name.
    #[new(default)]
    pub sequence_name_by_id: HashMap<SequenceId, SeqName>,
    /// Maps sequence name to sequence ID.
    #[new(default)]
    pub sequence_id_by_name: HashMap<SeqName, SequenceId>,
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
            sequence_name_by_id: HashMap::with_capacity(capacity),
            sequence_id_by_name: HashMap::with_capacity(capacity),
        }
    }

    /// Inserts a mapping from the sequence name to ID and back.
    pub fn insert(&mut self, sequence_name: SeqName, sequence_id: SequenceId) {
        self.sequence_id_by_name.insert(sequence_name, sequence_id);
        self.sequence_name_by_id.insert(sequence_id, sequence_name);
    }

    /// Returns the sequence name for the given ID.
    pub fn name(&self, sequence_id: SequenceId) -> Option<&SeqName> {
        self.sequence_name_by_id.get(&sequence_id)
    }

    /// Returns the sequence name for the given ID.
    pub fn id(&self, sequence_name: SeqName) -> Option<&SequenceId> {
        self.sequence_id_by_name.get(&sequence_name)
    }

    /// Returns the number of sequence mappings.
    pub fn len(&self) -> usize {
        self.sequence_name_by_id.len()
    }

    /// Returns if the mappings is empty.
    pub fn is_empty(&self) -> bool {
        self.sequence_name_by_id.is_empty()
    }
}

impl<SeqName> FromIterator<(SeqName, SequenceId)> for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    fn from_iter<T: IntoIterator<Item = (SeqName, SequenceId)>>(
        iter: T,
    ) -> SequenceIdMappings<SeqName> {
        let mut sequence_id_mappings = SequenceIdMappings::default();
        sequence_id_mappings.extend(iter);
        sequence_id_mappings
    }
}

impl<SeqName> Extend<(SeqName, SequenceId)> for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (SeqName, SequenceId)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(sequence_name, sequence_id)| {
            self.insert(sequence_name, sequence_id);
        });
    }
}

impl<'a, SeqName> Extend<(&'a SeqName, &'a SequenceId)> for SequenceIdMappings<SeqName>
where
    SeqName: config::SequenceName,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a SeqName, &'a SequenceId)>>(&mut self, iter: T) {
        iter.into_iter()
            .map(|(sequence_name, sequence_id)| (*sequence_name, *sequence_id))
            .for_each(|(sequence_name, sequence_id)| {
                self.insert(sequence_name, sequence_id);
            });
    }
}
