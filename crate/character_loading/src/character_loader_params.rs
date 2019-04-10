use amethyst::assets::{AssetStorage, Loader};
use character_model::config::CharacterSequenceId;
use derivative::Derivative;
use sequence_model::loaded::ControlTransitionsSequence;

/// Resources needed to load a character.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct CharacterLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `ControlTransitionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub control_transitions_sequence_assets:
        &'s AssetStorage<ControlTransitionsSequence<CharacterSequenceId>>,
}
