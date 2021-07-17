// use sequence_model_derive::frame_component_data;

use crate::loaded::{InputReaction, InputReactionsHandle};
use sequence_model_spi::loaded::FrameComponentData;

/// Sequence of input reactions.
// #[frame_component_data(InputReactionsHandle<InputReaction>)]
#[derive(
    asset_derive::Asset, derive_deref::Deref, derive_deref::DerefMut, Clone, Debug, PartialEq,
)]
pub struct InputReactionsSequence<IR = InputReaction>(FrameComponentData<InputReactionsHandle<IR>>)
where
    IR: Send + Sync + 'static;

impl<IR> InputReactionsSequence<IR>
where
    IR: Send + Sync + 'static,
{
    /// Returns a new `InputReactionsSequence`.
    pub fn new(sequence: std::vec::Vec<InputReactionsHandle<IR>>) -> Self {
        InputReactionsSequence(sequence_model_spi::loaded::FrameComponentData::<
            InputReactionsHandle<IR>,
        >::new(sequence))
    }
}

// Manually implement `Default` because the component type may not, and the
// `Default` derive imposes a `Default` bound on type parameters.
impl<IR> Default for InputReactionsSequence<IR>
where
    IR: Send + Sync + 'static,
{
    fn default() -> Self {
        InputReactionsSequence(sequence_model_spi::loaded::FrameComponentData::<
            InputReactionsHandle<IR>,
        >::new(std::vec::Vec::default()))
    }
}

impl<IR> sequence_model_spi::loaded::ComponentDataExt for InputReactionsSequence<IR>
where
    IR: Send + Sync + 'static,
{
    type Component = InputReactionsHandle<IR>;

    fn new(sequence: std::vec::Vec<InputReactionsHandle<IR>>) -> Self {
        InputReactionsSequence::<IR>::new(sequence)
    }

    fn to_owned(component: &Self::Component) -> Self::Component {
        component.clone()
    }
}
