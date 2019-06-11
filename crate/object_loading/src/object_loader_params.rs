use amethyst::{
    assets::{AssetStorage, Loader},
    renderer::sprite::SpriteSheetHandle,
};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use derivative::Derivative;
use sequence_model::loaded::{ComponentSequences, WaitSequence};
use sprite_model::loaded::SpriteRenderSequence;

use crate::ObjectLoaderSystemData;

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
    /// `WaitSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: &'s AssetStorage<WaitSequence>,
    /// `SpriteRenderSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: &'s AssetStorage<SpriteRenderSequence>,
    /// `BodySequence`s assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: &'s AssetStorage<BodySequence>,
    /// `InteractionsSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: &'s AssetStorage<InteractionsSequence>,
    /// `AssetStorage` for `Body`s.
    #[derivative(Debug = "ignore")]
    pub body_assets: &'s AssetStorage<Body>,
    /// `AssetStorage` for `Interactions`s.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: &'s AssetStorage<Interactions>,
    /// Handles to the sprite sheets for this `Object`.
    pub sprite_sheet_handles: &'s [SpriteSheetHandle],
}

impl<'s> From<(&'s ObjectLoaderSystemData<'s>, &'s [SpriteSheetHandle])>
    for ObjectLoaderParams<'s>
{
    fn from(
        (object_loader_system_data, sprite_sheet_handles): (
            &'s ObjectLoaderSystemData<'s>,
            &'s [SpriteSheetHandle],
        ),
    ) -> Self {
        let ObjectLoaderSystemData {
            ref loader,
            ref component_sequences_assets,
            ref wait_sequence_assets,
            ref sprite_render_sequence_assets,
            ref body_sequence_assets,
            ref interactions_sequence_assets,
            ref body_assets,
            ref interactions_assets,
        } = object_loader_system_data;

        ObjectLoaderParams {
            loader,
            component_sequences_assets,
            wait_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            body_assets,
            interactions_assets,
            sprite_sheet_handles,
        }
    }
}
