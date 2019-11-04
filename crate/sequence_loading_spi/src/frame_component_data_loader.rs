use amethyst::{
    assets::{Asset, AssetStorage, Handle, Loader},
    ecs::Component,
};
use sequence_model_spi::loaded::ComponentDataExt;

/// Loads frame component data, typically a sequence of a particular frame component.
pub trait FrameComponentDataLoader {
    /// The component that changes per frame.
    type Component: Component;
    /// The data type that holds the frame components.
    type ComponentData: Asset
        + ComponentDataExt<Component = Self::Component>
        + Send
        + Sync
        + 'static;

    /// Loads frame component `Sequence` as an asset and returns its handle.
    fn load<SequenceIterator, FrameRef, FnFrameToComponent>(
        loader: &Loader,
        frame_component_data_assets: &AssetStorage<Self::ComponentData>,
        fn_frame_to_component: FnFrameToComponent,
        sequence_iterator: SequenceIterator,
    ) -> Handle<Self::ComponentData>
    where
        SequenceIterator: Iterator<Item = FrameRef>,
        FnFrameToComponent: Fn(FrameRef) -> Self::Component,
        Self::ComponentData: Asset<Data = Self::ComponentData>,
    {
        let frame_component_data = <Self::ComponentData as ComponentDataExt>::new(
            sequence_iterator
                .map(fn_frame_to_component)
                .collect::<Vec<Self::Component>>(),
        );

        loader.load_from_data(frame_component_data, (), frame_component_data_assets)
    }
}
