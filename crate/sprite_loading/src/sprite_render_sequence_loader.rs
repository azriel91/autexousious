use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    renderer::SpriteRender,
};
use derivative::Derivative;
use sequence_loading_spi::FrameComponentDataLoader;
use sprite_model::loaded::SpriteRenderSequence;

/// Loads `SpriteRenderSequence`s from `Sequence` types whose `Frame`s contain a
/// `SpriteRender`.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct SpriteRenderSequenceLoader<'s> {
    /// `Loader`.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: &'s AssetStorage<SpriteRenderSequence>,
}

impl<'s> SpriteRenderSequenceLoader<'s> {
    /// Loads a `SpriteRenderSequence` and returns its handle.
    ///
    /// This is similar to calling the `FrameComponentDataLoader::load` trait
    /// method, with the difference that the resources are stored by an
    /// instantiation of this type, so they do not need to be passed in when
    /// this method is called.
    pub fn load<SequenceIterator, FrameRef, FnFrameToComponent>(
        &self,
        fn_frame_to_component: FnFrameToComponent,
        sequence_iterator: SequenceIterator,
    ) -> Handle<SpriteRenderSequence>
    where
        SequenceIterator: Iterator<Item = FrameRef>,
        FnFrameToComponent: Fn(FrameRef) -> SpriteRender,
    {
        <Self as FrameComponentDataLoader>::load(
            self.loader,
            self.sprite_render_sequence_assets,
            fn_frame_to_component,
            sequence_iterator,
        )
    }
}

impl<'s> FrameComponentDataLoader for SpriteRenderSequenceLoader<'s> {
    type Component = SpriteRender;
    type ComponentData = SpriteRenderSequence;
}
