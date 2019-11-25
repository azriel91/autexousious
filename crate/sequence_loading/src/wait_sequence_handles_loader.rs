use sequence_loading_spi::SequenceComponentDataLoader;
use sequence_model::{
    config::Wait,
    loaded::{WaitSequenceHandle, WaitSequenceHandles},
};

use crate::WaitSequenceLoader;

/// Loads `WaitSequenceHandle`s from collections of sequences that contain `Wait` values.
#[derive(Debug)]
pub struct WaitSequenceHandlesLoader<'s> {
    /// `WaitSequenceLoader`.
    pub wait_sequence_loader: WaitSequenceLoader<'s>,
}

impl<'s> WaitSequenceHandlesLoader<'s> {
    /// Loads `WaitSequenceHandles`.
    ///
    /// This is similar to calling the `SequenceComponentDataLoader::load` trait method, with the
    /// difference that the resources are stored by an instantiation of this type, so they do not
    /// need to be passed in when this method is called.
    pub fn items_to_datas<
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
    ) -> WaitSequenceHandles
    where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: 'seq_ref,
        FnSequencesToSequenceIterator: Fn(SequenceRef) -> SequenceIterator,
        FrameRef: AsRef<Wait> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.wait_sequence_loader.load(
                    |frame| *AsRef::<Wait>::as_ref(&frame),
                    fn_sequences_to_sequence_iterator(sequence_ref),
                )
            },
            sequences_iterator,
        )
    }
}

impl<'s> SequenceComponentDataLoader for WaitSequenceHandlesLoader<'s> {
    type Component = WaitSequenceHandle;
    type ComponentData = WaitSequenceHandles;
}
