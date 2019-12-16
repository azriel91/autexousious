use amethyst::assets::{AssetStorage, Loader};
use derivative::Derivative;
use input_reaction_model::loaded::{InputReaction, InputReactions, InputReactionsSequence};
use typename::TypeName;

/// Resources needed to load an `InputReactionsSequence`.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct IrsLoaderParams<'s, IRR>
where
    IRR: Send + Sync + TypeName + 'static,
{
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `InputReactions` assets.
    #[derivative(Debug = "ignore")]
    pub input_reactions_assets: &'s AssetStorage<InputReactions<InputReaction<IRR>>>,
    /// `InputReactionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub input_reactions_sequence_assets:
        &'s AssetStorage<InputReactionsSequence<InputReaction<IRR>>>,
}
