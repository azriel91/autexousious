use asset_model::loaded::AssetId;
use derivative::Derivative;
use sequence_loading_spi::SequenceComponentDataLoader;
use sequence_model::{
    config::{Sequence, SequenceName, Wait},
    loaded::{AssetSequenceEndTransitions, SequenceEndTransition, SequenceEndTransitions},
};

use crate::SequenceEndTransitionMapper;

/// Loads `SequenceEndTransition`s from sequences.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct SequenceEndTransitionsLoader<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// `SequenceEndTransitionMapper`.
    pub sequence_end_transition_mapper: SequenceEndTransitionMapper<'s, SeqName>,
    /// `AssetSequenceEndTransitions`.
    pub asset_sequence_end_transitions: &'s mut AssetSequenceEndTransitions,
}

impl<'s, SeqName> SequenceEndTransitionsLoader<'s, SeqName>
where
    SeqName: SequenceName,
{
    /// Loads `SequenceEndTransitions`.
    ///
    /// This is similar to calling the `SequenceComponentDataLoader::load` trait method, with the
    /// difference that the resources are stored by an instantiation of this type, so they do not
    /// need to be passed in when this method is called.
    pub fn load<SequencesIterator, SequenceRef, Frm>(
        &mut self,
        sequences_iterator: SequencesIterator,
        asset_id: AssetId,
    ) where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: AsRef<Sequence<SeqName, Frm>>,
        Frm: AsRef<Wait>,
    {
        let sequence_end_transitions = <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.sequence_end_transition_mapper
                    .map(asset_id, sequence_ref)
            },
            sequences_iterator,
        );
        self.asset_sequence_end_transitions
            .insert(asset_id, sequence_end_transitions);
    }
}

impl<'s, SeqName> SequenceComponentDataLoader for SequenceEndTransitionsLoader<'s, SeqName>
where
    SeqName: SequenceName,
{
    type Component = SequenceEndTransition;
    type ComponentData = SequenceEndTransitions;
}
