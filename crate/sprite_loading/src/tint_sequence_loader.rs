use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    renderer::{palette::Srgba, resources::Tint},
};
use derivative::Derivative;
use sequence_loading_spi::FrameComponentDataLoader;
use sprite_model::{config, loaded::TintSequence};

/// Loads `TintSequence`s from `Sequence` types whose `Frame`s contain a `Tint` value.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TintSequenceLoader<'s> {
    /// `Loader`.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `TintSequence` assets.
    #[derivative(Debug = "ignore")]
    pub tint_sequence_assets: &'s AssetStorage<TintSequence>,
}

impl<'s> TintSequenceLoader<'s> {
    /// Loads a `TintSequence` and returns its handle.
    ///
    /// This is similar to calling the `FrameComponentDataLoader::load` trait method, with the
    /// difference that the resources are stored by an instantiation of this type, so they do not
    /// need to be passed in when this method is called.
    pub fn load<SequenceIterator, FrameRef>(
        &self,
        sequence_iterator: SequenceIterator,
    ) -> Handle<TintSequence>
    where
        SequenceIterator: Iterator<Item = FrameRef>,
        FrameRef: AsRef<config::Tint>,
    {
        <Self as FrameComponentDataLoader>::load(
            self.loader,
            self.tint_sequence_assets,
            Self::frame_to_component,
            sequence_iterator,
        )
    }

    /// Maps the frame to the component.
    ///
    /// # Parameters
    ///
    /// * `frame_ref`: Reference to the sequence.
    pub fn frame_to_component<FrameRef>(frame_ref: FrameRef) -> Tint
    where
        FrameRef: AsRef<config::Tint>,
    {
        let tint = *AsRef::<config::Tint>::as_ref(&frame_ref);
        Tint(Srgba::new(tint.r, tint.g, tint.b, tint.a))
    }
}

impl<'s> FrameComponentDataLoader for TintSequenceLoader<'s> {
    type Component = Tint;
    type ComponentData = TintSequence;
}
