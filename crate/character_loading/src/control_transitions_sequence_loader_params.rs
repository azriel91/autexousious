use amethyst::assets::{AssetStorage, Loader};
use character_model::loaded::{CharacterControlTransitions, CharacterControlTransitionsSequence};
use derivative::Derivative;

/// Resources needed to load a control transitions sequence.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ControlTransitionsSequenceLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: &'s AssetStorage<CharacterControlTransitions>,
    /// `CharacterControlTransitionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_sequence_assets:
        &'s AssetStorage<CharacterControlTransitionsSequence>,
}
