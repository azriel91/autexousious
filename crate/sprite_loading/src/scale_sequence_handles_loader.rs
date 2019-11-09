use asset_model::loaded::AssetId;
use derivative::Derivative;
use sequence_loading_spi::SequenceComponentDataLoader;
use sprite_model::{
    config::Scale,
    loaded::{AssetScaleSequenceHandles, ScaleSequenceHandle, ScaleSequenceHandles},
};

use crate::ScaleSequenceLoader;

/// Loads `ScaleSequenceHandle`s from collections of sequences that contain `Scale` values.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ScaleSequenceHandlesLoader<'s> {
    /// `ScaleSequenceLoader`.
    pub scale_sequence_loader: ScaleSequenceLoader<'s>,
    /// `AssetScaleSequenceHandles`.
    pub asset_scale_sequence_handles: &'s mut AssetScaleSequenceHandles,
}

impl<'s> ScaleSequenceHandlesLoader<'s> {
    /// Loads `ScaleSequenceHandles`.
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
        FrameRef: AsRef<Scale> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        let scale_sequence_handles = <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.scale_sequence_loader
                    .load(fn_sequences_to_sequence_iterator(sequence_ref))
            },
            sequences_iterator,
        );
        self.asset_scale_sequence_handles
            .insert(asset_id, scale_sequence_handles);
    }
}

impl<'s> SequenceComponentDataLoader for ScaleSequenceHandlesLoader<'s> {
    type Component = ScaleSequenceHandle;
    type ComponentData = ScaleSequenceHandles;
}
