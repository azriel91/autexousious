use amethyst::{
    assets::Handle,
    renderer::{SpriteRender, SpriteSheet},
};
use asset_model::loaded::AssetId;
use sequence_loading_spi::SequenceComponentDataLoader;
use sprite_model::{
    config::SpriteRef,
    loaded::{
        AssetSpriteRenderSequenceHandles, SpriteRenderSequenceHandle, SpriteRenderSequenceHandles,
    },
};

use crate::SpriteRenderSequenceLoader;

/// Loads `SpriteRenderSequenceHandle`s from collections of sequences that contain `SpriteRender` values.
#[derive(Debug)]
pub struct SpriteRenderSequenceHandlesLoader<'s> {
    /// `SpriteRenderSequenceLoader`.
    pub sprite_render_sequence_loader: SpriteRenderSequenceLoader<'s>,
    /// `AssetSpriteRenderSequenceHandles`.
    pub asset_sprite_render_sequence_handles: &'s mut AssetSpriteRenderSequenceHandles,
}

impl<'s> SpriteRenderSequenceHandlesLoader<'s> {
    /// Loads `SpriteRenderSequenceHandles`.
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
        sprite_sheet_handles: &[Handle<SpriteSheet>],
    ) where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: 'seq_ref,
        FnSequencesToSequenceIterator: Fn(SequenceRef) -> SequenceIterator,
        FrameRef: AsRef<SpriteRef> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        let sprite_render_sequence_handles = self.items_to_datas(
            sequences_iterator,
            fn_sequences_to_sequence_iterator,
            sprite_sheet_handles,
        );
        self.asset_sprite_render_sequence_handles
            .insert(asset_id, sprite_render_sequence_handles);
    }

    /// Maps items to `WaitSequenceHandles`.
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
        sprite_sheet_handles: &[Handle<SpriteSheet>],
    ) -> SpriteRenderSequenceHandles
    where
        SequencesIterator: Iterator<Item = SequenceRef>,
        SequenceRef: 'seq_ref,
        FnSequencesToSequenceIterator: Fn(SequenceRef) -> SequenceIterator,
        FrameRef: AsRef<SpriteRef> + 'frame_ref,
        SequenceIterator: Iterator<Item = FrameRef>,
    {
        <Self as SequenceComponentDataLoader>::load(
            |sequence_ref| {
                self.sprite_render_sequence_loader.load(
                    |frame| {
                        let sprite_ref = AsRef::<SpriteRef>::as_ref(&frame);
                        let sprite_sheet = sprite_sheet_handles[sprite_ref.sheet].clone();
                        let sprite_number = sprite_ref.index;

                        SpriteRender {
                            sprite_sheet,
                            sprite_number,
                        }
                    },
                    fn_sequences_to_sequence_iterator(sequence_ref),
                )
            },
            sequences_iterator,
        )
    }
}

impl<'s> SequenceComponentDataLoader for SpriteRenderSequenceHandlesLoader<'s> {
    type Component = SpriteRenderSequenceHandle;
    type ComponentData = SpriteRenderSequenceHandles;
}
