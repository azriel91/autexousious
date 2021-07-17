use sequence_loading_spi::SequenceComponentDataLoader;
use sprite_model::{
    config,
    loaded::{TintSequenceHandle, TintSequenceHandles},
};

use crate::TintSequenceLoader;

/// Loads `TintSequenceHandle`s from collections of sequences that contain
/// `Tint` values.
#[derive(Debug)]
pub struct TintSequenceHandlesLoader<'s> {
    /// `TintSequenceLoader`.
    pub tint_sequence_loader: TintSequenceLoader<'s>,
}

impl<'s> TintSequenceHandlesLoader<'s> {
    /// Loads `TintSequenceHandles`.
    ///
    /// This is similar to calling the `SequenceComponentDataLoader::load` trait
    /// method, with the difference that the resources are stored by an
    /// instantiation of this type, so they do not need to be passed in when
    /// this method is called.
    pub fn items_to_datas<
        'seq_ref,
        'frame_ref: 'seq_ref,
        SequencesIterator,
        SequenceRef,
        FnSequencesToSequenceIterator,
        SequenceIterator,
        FrameRef,
    >(
        &self,
        sequences_iterator: SequencesIterator,
        fn_sequences_to_sequence_iterator: FnSequencesToSequenceIterator,
    ) -> TintSequenceHandles
    where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: 'seq_ref,
        FnSequencesToSequenceIterator: Fn(SequenceRef) -> SequenceIterator,
        FrameRef: AsRef<config::Tint> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.tint_sequence_loader
                    .load(fn_sequences_to_sequence_iterator(sequence_ref))
            },
            sequences_iterator,
        )
    }
}

impl<'s> SequenceComponentDataLoader for TintSequenceHandlesLoader<'s> {
    type Component = TintSequenceHandle;
    type ComponentData = TintSequenceHandles;
}
