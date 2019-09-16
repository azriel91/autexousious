use derivative::Derivative;

use crate::CtsLoaderParams;

/// Resources needed to load a control transitions sequence.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct CharacterLoaderParams<'s> {
    /// `CtsLoaderParams`.
    #[derivative(Debug = "ignore")]
    pub cts_loader_params: CtsLoaderParams<'s>,
}
