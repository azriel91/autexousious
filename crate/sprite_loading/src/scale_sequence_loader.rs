use amethyst::assets::{AssetStorage, Handle, Loader};
use derivative::Derivative;
use sequence_loading_spi::FrameComponentDataLoader;
use sprite_model::{config::Scale, loaded::ScaleSequence};

/// Loads `ScaleSequence`s from `Sequence` types whose `Frame`s contain a
/// `Scale` value.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ScaleSequenceLoader<'s> {
    /// `Loader`.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `ScaleSequence` assets.
    #[derivative(Debug = "ignore")]
    pub scale_sequence_assets: &'s AssetStorage<ScaleSequence>,
}

impl<'s> ScaleSequenceLoader<'s> {
    /// Loads a `ScaleSequence` and returns its handle.
    ///
    /// This is similar to calling the `FrameComponentDataLoader::load` trait
    /// method, with the difference that the resources are stored by an
    /// instantiation of this type, so they do not need to be passed in when
    /// this method is called.
    pub fn load<SequenceIterator, FrameRef>(
        &self,
        sequence_iterator: SequenceIterator,
    ) -> Handle<ScaleSequence>
    where
        SequenceIterator: Iterator<Item = FrameRef>,
        FrameRef: AsRef<Scale>,
    {
        <Self as FrameComponentDataLoader>::load(
            self.loader,
            self.scale_sequence_assets,
            Self::frame_to_component,
            sequence_iterator,
        )
    }

    /// Maps the frame to the component.
    ///
    /// # Parameters
    ///
    /// * `frame_ref`: Reference to the frame.
    pub fn frame_to_component<FrameRef>(frame_ref: FrameRef) -> Scale
    where
        FrameRef: AsRef<Scale>,
    {
        *AsRef::<Scale>::as_ref(&frame_ref)
    }
}

impl<'s> FrameComponentDataLoader for ScaleSequenceLoader<'s> {
    type Component = Scale;
    type ComponentData = ScaleSequence;
}
