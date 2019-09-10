use amethyst::{
    assets::{AssetStorage, Loader},
    audio::Source,
    renderer::sprite::SpriteSheetHandle,
};
use audio_model::loaded::SourceSequence;
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use derivative::Derivative;
use kinematic_model::loaded::ObjectAccelerationSequence;
use sequence_model::loaded::WaitSequence;
use spawn_model::{config::Spawns, loaded::SpawnsSequence};
use sprite_model::loaded::SpriteRenderSequence;

use crate::ObjectLoaderSystemData;

/// Resources needed to load an object.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ObjectLoaderParams<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: &'s Loader,
    /// `WaitSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: &'s AssetStorage<WaitSequence>,
    /// `Source`s assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: &'s AssetStorage<Source>,
    /// `SourceSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub source_sequence_assets: &'s AssetStorage<SourceSequence>,
    /// `ObjectAccelerationSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub object_acceleration_sequence_assets: &'s AssetStorage<ObjectAccelerationSequence>,
    /// `SpriteRenderSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: &'s AssetStorage<SpriteRenderSequence>,
    /// `BodySequence`s assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: &'s AssetStorage<BodySequence>,
    /// `InteractionsSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: &'s AssetStorage<InteractionsSequence>,
    /// `SpawnsSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub spawns_sequence_assets: &'s AssetStorage<SpawnsSequence>,
    /// `Body` assets.
    #[derivative(Debug = "ignore")]
    pub body_assets: &'s AssetStorage<Body>,
    /// `Interactions` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: &'s AssetStorage<Interactions>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: &'s AssetStorage<Spawns>,
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
            ref wait_sequence_assets,
            ref source_assets,
            ref source_sequence_assets,
            ref object_acceleration_sequence_assets,
            ref sprite_render_sequence_assets,
            ref body_sequence_assets,
            ref interactions_sequence_assets,
            ref spawns_sequence_assets,
            ref body_assets,
            ref interactions_assets,
            ref spawns_assets,
        } = object_loader_system_data;

        ObjectLoaderParams {
            loader,
            wait_sequence_assets,
            source_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            body_assets,
            interactions_assets,
            spawns_assets,
            sprite_sheet_handles,
        }
    }
}
