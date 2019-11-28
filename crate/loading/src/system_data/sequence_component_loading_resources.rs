use amethyst::{
    assets::AssetStorage,
    audio::Source,
    ecs::{Read, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::AssetItemIds, play::AssetWorld};
use audio_model::loaded::SourceSequence;
use character_model::loaded::{CharacterControlTransitions, CharacterCts};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use derivative::Derivative;
use game_input_model::InputConfig;
use kinematic_model::loaded::ObjectAccelerationSequence;
use map_model::loaded::{AssetMapBounds, AssetMargins};
use sequence_model::loaded::WaitSequence;
use spawn_model::loaded::{Spawns, SpawnsSequence};
use sprite_model::loaded::{ScaleSequence, SpriteRenderSequence, TintSequence};

use crate::{DefinitionLoadingResourcesRead, IdMappingResourcesRead, TextureLoadingResourcesRead};

/// `SequenceComponentLoadingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentLoadingResources<'s> {
    /// `DefinitionLoadingResourcesRead`.
    pub definition_loading_resources_read: DefinitionLoadingResourcesRead<'s>,
    /// `IdMappingResourcesRead`.
    pub id_mapping_resources_read: IdMappingResourcesRead<'s>,
    /// `TextureLoadingResourcesRead`.
    pub texture_loading_resources_read: TextureLoadingResourcesRead<'s>,

    /// `AssetWorld` for loaded item components.
    #[derivative(Debug = "ignore")]
    pub asset_world: Write<'s, AssetWorld>,
    /// `AssetItemIds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_item_ids: Write<'s, AssetItemIds>,
    /// `InputConfig` resource.
    #[derivative(Debug = "ignore")]
    pub input_config: Read<'s, InputConfig>,

    /// `Source`s assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: Read<'s, AssetStorage<Source>>,
    /// `Body` assets.
    #[derivative(Debug = "ignore")]
    pub body_assets: Read<'s, AssetStorage<Body>>,
    /// `Interactions` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: Read<'s, AssetStorage<Interactions>>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,

    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SourceSequence` assets.
    #[derivative(Debug = "ignore")]
    pub source_sequence_assets: Read<'s, AssetStorage<SourceSequence>>,
    /// `ObjectAccelerationSequence` assets.
    #[derivative(Debug = "ignore")]
    pub object_acceleration_sequence_assets: Read<'s, AssetStorage<ObjectAccelerationSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
    /// `BodySequence` assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: Read<'s, AssetStorage<BodySequence>>,
    /// `InteractionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: Read<'s, AssetStorage<InteractionsSequence>>,
    /// `SpawnsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_sequence_assets: Read<'s, AssetStorage<SpawnsSequence>>,

    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: Read<'s, AssetStorage<CharacterControlTransitions>>,
    /// `CharacterCts` assets.
    #[derivative(Debug = "ignore")]
    pub character_cts_assets: Read<'s, AssetStorage<CharacterCts>>,

    /// `TintSequence` assets.
    #[derivative(Debug = "ignore")]
    pub tint_sequence_assets: Read<'s, AssetStorage<TintSequence>>,
    /// `ScaleSequence` assets.
    #[derivative(Debug = "ignore")]
    pub scale_sequence_assets: Read<'s, AssetStorage<ScaleSequence>>,

    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Write<'s, AssetMapBounds>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Write<'s, AssetMargins>,
}

/// `SequenceComponentLoadingResourcesRead`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentLoadingResourcesRead<'s> {
    /// `Source`s assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: Read<'s, AssetStorage<Source>>,
    /// `Body` assets.
    #[derivative(Debug = "ignore")]
    pub body_assets: Read<'s, AssetStorage<Body>>,
    /// `Interactions` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: Read<'s, AssetStorage<Interactions>>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,

    /// `AssetWorld` for loaded item components.
    #[derivative(Debug = "ignore")]
    pub asset_world: Read<'s, AssetWorld>,
    /// `AssetItemIds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_item_ids: Read<'s, AssetItemIds>,

    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SourceSequence` assets.
    #[derivative(Debug = "ignore")]
    pub source_sequence_assets: Read<'s, AssetStorage<SourceSequence>>,
    /// `ObjectAccelerationSequence` assets.
    #[derivative(Debug = "ignore")]
    pub object_acceleration_sequence_assets: Read<'s, AssetStorage<ObjectAccelerationSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
    /// `BodySequence` assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: Read<'s, AssetStorage<BodySequence>>,
    /// `InteractionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: Read<'s, AssetStorage<InteractionsSequence>>,
    /// `SpawnsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_sequence_assets: Read<'s, AssetStorage<SpawnsSequence>>,

    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: Read<'s, AssetStorage<CharacterControlTransitions>>,
    /// `CharacterCts` assets.
    #[derivative(Debug = "ignore")]
    pub character_cts_assets: Read<'s, AssetStorage<CharacterCts>>,

    /// `TintSequence` assets.
    #[derivative(Debug = "ignore")]
    pub tint_sequence_assets: Read<'s, AssetStorage<TintSequence>>,
    /// `ScaleSequence` assets.
    #[derivative(Debug = "ignore")]
    pub scale_sequence_assets: Read<'s, AssetStorage<ScaleSequence>>,

    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Read<'s, AssetMapBounds>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
}
