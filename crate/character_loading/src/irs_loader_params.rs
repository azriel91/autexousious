use amethyst::assets::{AssetStorage, Loader};
use character_model::loaded::{CharacterInputReactions, CharacterIrs};
use derivative::Derivative;

/// Resources needed to load a control transitions sequence.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct IrsLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `CharacterInputReactions` assets.
    #[derivative(Debug = "ignore")]
    pub character_input_reactions_assets: &'s AssetStorage<CharacterInputReactions>,
    /// `CharacterIrs` assets.
    #[derivative(Debug = "ignore")]
    pub character_irs_assets: &'s AssetStorage<CharacterIrs>,
}
