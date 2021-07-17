use amethyst::assets::{AssetStorage, Handle, Loader};
use derivative::Derivative;
use sequence_loading_spi::FrameComponentDataLoader;
use sequence_model::{config::Wait, loaded::WaitSequence};

/// Loads `WaitSequence`s from `Sequence` types whose `Frame`s contain a `Wait`
/// value.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct WaitSequenceLoader<'s> {
    /// `Loader`.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: &'s AssetStorage<WaitSequence>,
}

impl<'s> WaitSequenceLoader<'s> {
    /// Loads a `WaitSequence` and returns its handle.
    ///
    /// This is similar to calling the `FrameComponentDataLoader::load` trait
    /// method, with the difference that the resources are stored by an
    /// instantiation of this type, so they do not need to be passed in when
    /// this method is called.
    pub fn load<SequenceIterator, FrameRef, FnFrameToComponent>(
        &self,
        fn_frame_to_component: FnFrameToComponent,
        sequence_iterator: SequenceIterator,
    ) -> Handle<WaitSequence>
    where
        SequenceIterator: Iterator<Item = FrameRef>,
        FnFrameToComponent: Fn(FrameRef) -> Wait,
    {
        <Self as FrameComponentDataLoader>::load(
            self.loader,
            self.wait_sequence_assets,
            fn_frame_to_component,
            sequence_iterator,
        )
    }
}

impl<'s> FrameComponentDataLoader for WaitSequenceLoader<'s> {
    type Component = Wait;
    type ComponentData = WaitSequence;
}
