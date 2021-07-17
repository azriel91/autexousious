use asset_model::config::AssetSlug;
use sequence_loading_spi::SequenceComponentDataLoader;
use sequence_model::{
    config::{Sequence, SequenceName, Wait},
    loaded::{SequenceEndTransition, SequenceEndTransitions, SequenceIdMappings},
};

use crate::SequenceEndTransitionMapper;

/// Loads `SequenceEndTransition`s from sequences.
#[derive(Debug)]
pub struct SequenceEndTransitionsLoader<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// `SequenceIdMappings<SeqName>`.
    pub sequence_id_mappings: &'s SequenceIdMappings<SeqName>,
}

impl<'s, SeqName> SequenceEndTransitionsLoader<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// Loads `SequenceEndTransitions`.
    ///
    /// This is similar to calling the `SequenceComponentDataLoader::load` trait
    /// method, with the difference that the resources are stored by an
    /// instantiation of this type, so they do not need to be passed in when
    /// this method is called.
    pub fn items_to_datas<SequencesIterator, SequenceRef, Frm>(
        &self,
        sequences_iterator: SequencesIterator,
        asset_slug: &AssetSlug,
    ) -> SequenceEndTransitions
    where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: AsRef<Sequence<SeqName, Frm>>,
        Frm: AsRef<Wait>,
    {
        <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                SequenceEndTransitionMapper::map(
                    self.sequence_id_mappings,
                    asset_slug,
                    sequence_ref,
                )
            },
            sequences_iterator,
        )
    }
}

impl<'s, SeqName> SequenceComponentDataLoader for SequenceEndTransitionsLoader<'s, SeqName>
where
    SeqName: SequenceName,
{
    type Component = SequenceEndTransition;
    type ComponentData = SequenceEndTransitions;
}
