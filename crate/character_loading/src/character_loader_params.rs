use derivative::Derivative;

use crate::ControlTransitionsSequenceLoaderParams;

/// Resources needed to load a control transitions sequence.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct CharacterLoaderParams<'s> {
    /// `ControlTransitionsSequenceLoaderParams`.
    #[derivative(Debug = "ignore")]
    pub control_transitions_sequence_loader_params: ControlTransitionsSequenceLoaderParams<'s>,
}
