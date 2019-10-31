use asset_model::loaded::AssetId;
use derivative::Derivative;
use sequence_loading_spi::SequenceComponentDataLoader;
use sequence_model::{
    config::Wait,
    loaded::{AssetWaitSequenceHandles, WaitSequenceHandle, WaitSequenceHandles},
};

use crate::WaitSequenceLoader;

/// Loads `WaitSequenceHandle`s from collections of sequences that contain `Wait` values.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct WaitSequenceHandlesLoader<'s> {
    /// `WaitSequenceLoader`.
    pub wait_sequence_loader: WaitSequenceLoader<'s>,
    /// `AssetWaitSequenceHandles`.
    pub asset_wait_sequence_handles: &'s mut AssetWaitSequenceHandles,
}

impl<'s> WaitSequenceHandlesLoader<'s> {
    /// Loads `WaitSequenceHandles`.
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
        FrameRef: AsRef<Wait> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        let wait_sequence_handles = <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.wait_sequence_loader.load(
                    |frame| *AsRef::<Wait>::as_ref(&frame),
                    fn_sequences_to_sequence_iterator(sequence_ref),
                )
            },
            sequences_iterator,
        );
        self.asset_wait_sequence_handles
            .insert(asset_id, wait_sequence_handles);
    }
}

impl<'s> SequenceComponentDataLoader for WaitSequenceHandlesLoader<'s> {
    type Component = WaitSequenceHandle;
    type ComponentData = WaitSequenceHandles;
}
