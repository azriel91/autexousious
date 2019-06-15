use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::{Read, ReadExpect},
};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use derivative::Derivative;
use sequence_model::loaded::WaitSequence;
use shred_derive::SystemData;
use sprite_model::loaded::SpriteRenderSequence;

/// Resources needed to load an object.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectLoaderSystemData<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `WaitSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SpriteRenderSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
    /// `BodySequence`s assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: Read<'s, AssetStorage<BodySequence>>,
    /// `InteractionsSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: Read<'s, AssetStorage<InteractionsSequence>>,
    /// `AssetStorage` for `Body`s.
    #[derivative(Debug = "ignore")]
    pub body_assets: Read<'s, AssetStorage<Body>>,
    /// `AssetStorage` for `Interactions`s.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: Read<'s, AssetStorage<Interactions>>,
}
