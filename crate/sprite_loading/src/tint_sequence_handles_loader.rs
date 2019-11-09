use asset_model::loaded::AssetId;
use derivative::Derivative;
use sequence_loading_spi::SequenceComponentDataLoader;
use sprite_model::{
    config,
    loaded::{AssetTintSequenceHandles, TintSequenceHandle, TintSequenceHandles},
};

use crate::TintSequenceLoader;

/// Loads `TintSequenceHandle`s from collections of sequences that contain `Tint` values.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TintSequenceHandlesLoader<'s> {
    /// `TintSequenceLoader`.
    pub tint_sequence_loader: TintSequenceLoader<'s>,
    /// `AssetTintSequenceHandles`.
    pub asset_tint_sequence_handles: &'s mut AssetTintSequenceHandles,
}

impl<'s> TintSequenceHandlesLoader<'s> {
    /// Loads `TintSequenceHandles`.
    ///
    /// This is similar to calling the `SequenceComponentDataLoader::load` trait method, with the
    /// difference that the resources are stored by an instantiation of this type, so they do not
    /// need to be passed in when this method is called.
    pub fn load<
        'seq_ref,
        'frame_ref: 'seq_ref,
        SequencesIterator,
        SequenceRef,
        FnSequencesToSequenceIterator,
        SequenceIterator,
        FrameRef,
    >(
        &mut self,
        sequences_iterator: SequencesIterator,
        fn_sequences_to_sequence_iterator: FnSequencesToSequenceIterator,
        asset_id: AssetId,
    ) where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: 'seq_ref,
        FnSequencesToSequenceIterator: Fn(SequenceRef) -> SequenceIterator,
        FrameRef: AsRef<config::Tint> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        let tint_sequence_handles = <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.tint_sequence_loader
                    .load(fn_sequences_to_sequence_iterator(sequence_ref))
            },
            sequences_iterator,
        );
        self.asset_tint_sequence_handles
            .insert(asset_id, tint_sequence_handles);
    }
}

impl<'s> SequenceComponentDataLoader for TintSequenceHandlesLoader<'s> {
    type Component = TintSequenceHandle;
    type ComponentData = TintSequenceHandles;
}
