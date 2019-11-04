use amethyst::ecs::Component;
use sequence_model_spi::loaded::ComponentDataExt;

/// Loads sequence component data, typically a collection of a particular sequence component.
///
/// This is distinct from `FrameComponentDataLoader` as the sequence component is not necessarily
/// a `Handle<FrameComponentData>`.
pub trait SequenceComponentDataLoader {
    /// The component that changes per sequence.
    type Component: Component;
    /// The data type that holds the sequence components.
    type ComponentData: ComponentDataExt<Component = Self::Component> + Send + Sync + 'static;

    /// Loads and returns sequence component data.
    fn load<SequencesIterator, SequenceRef, FnSequenceToComponent>(
        fn_sequence_to_component: FnSequenceToComponent,
        sequence_iterator: SequencesIterator,
    ) -> Self::ComponentData
    where
        SequencesIterator: Iterator<Item = SequenceRef>,
        FnSequenceToComponent: Fn(SequenceRef) -> Self::Component,
    {
        <Self::ComponentData as ComponentDataExt>::new(
            sequence_iterator
                .map(fn_sequence_to_component)
                .collect::<Vec<Self::Component>>(),
        )
    }
}
