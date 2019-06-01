use amethyst::{
    assets::{AssetStorage, Loader},
    renderer::sprite::SpriteSheetHandle,
};
use collision_model::config::{Body, Interactions};
use derivative::Derivative;
use sequence_model::loaded::ComponentSequences;

/// Resources needed to load an object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ObjectLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `AssetStorage` for `ComponentSequences`s.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: &'s AssetStorage<ComponentSequences>,
    /// Handles to the sprite sheets for this `Object`.
    pub sprite_sheet_handles: &'s [SpriteSheetHandle],
    /// `AssetStorage` for `Body`s.
    #[derivative(Debug = "ignore")]
    pub body_assets: &'s AssetStorage<Body>,
    /// `AssetStorage` for `Interactions`s.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: &'s AssetStorage<Interactions>,
}
