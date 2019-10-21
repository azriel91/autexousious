use amethyst::{
    assets::AssetStorage,
    audio::Source,
    ecs::{Read, World, Write},
    shred::{ResourceId, SystemData},
};
use audio_model::loaded::{AssetSourceSequenceHandles, SourceSequence};
use character_model::loaded::{
    AssetCharacterCtsHandles, CharacterControlTransitions, CharacterCts,
};
use collision_model::{
    config::{Body, Interactions},
    loaded::{
        AssetBodySequenceHandles, AssetInteractionsSequenceHandles, BodySequence,
        InteractionsSequence,
    },
};
use derivative::Derivative;
use kinematic_model::loaded::{AssetObjectAccelerationSequenceHandles, ObjectAccelerationSequence};
use map_model::loaded::{AssetLayerPositions, AssetMapBounds, AssetMargins};
use sequence_model::loaded::{AssetSequenceEndTransitions, AssetWaitSequenceHandles, WaitSequence};
use spawn_model::loaded::{AssetSpawnsSequenceHandles, Spawns, SpawnsSequence};
use sprite_model::loaded::{AssetSpriteRenderSequenceHandles, SpriteRenderSequence};

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

    /// `AssetSequenceEndTransitions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_end_transitions: Write<'s, AssetSequenceEndTransitions>,
    /// `AssetWaitSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Write<'s, AssetWaitSequenceHandles>,
    /// `AssetSourceSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_source_sequence_handles: Write<'s, AssetSourceSequenceHandles>,
    /// `AssetObjectAccelerationSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_object_acceleration_sequence_handles:
        Write<'s, AssetObjectAccelerationSequenceHandles>,
    /// `AssetSpriteRenderSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_render_sequence_handles: Write<'s, AssetSpriteRenderSequenceHandles>,
    /// `AssetBodySequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_body_sequence_handles: Write<'s, AssetBodySequenceHandles>,
    /// `AssetInteractionsSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_interactions_sequence_handles: Write<'s, AssetInteractionsSequenceHandles>,
    /// `AssetSpawnsSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_spawns_sequence_handles: Write<'s, AssetSpawnsSequenceHandles>,

    /// `AssetCharacterCtsHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_cts_handles: Write<'s, AssetCharacterCtsHandles>,

    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Write<'s, AssetMapBounds>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Write<'s, AssetMargins>,
    /// `AssetLayerPositions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_layer_positions: Write<'s, AssetLayerPositions>,
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

    /// `AssetSequenceEndTransitions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_end_transitions: Read<'s, AssetSequenceEndTransitions>,
    /// `AssetWaitSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Read<'s, AssetWaitSequenceHandles>,
    /// `AssetSourceSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_source_sequence_handles: Read<'s, AssetSourceSequenceHandles>,
    /// `AssetObjectAccelerationSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_object_acceleration_sequence_handles:
        Read<'s, AssetObjectAccelerationSequenceHandles>,
    /// `AssetSpriteRenderSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_render_sequence_handles: Read<'s, AssetSpriteRenderSequenceHandles>,
    /// `AssetBodySequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_body_sequence_handles: Read<'s, AssetBodySequenceHandles>,
    /// `AssetInteractionsSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_interactions_sequence_handles: Read<'s, AssetInteractionsSequenceHandles>,
    /// `AssetSpawnsSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_spawns_sequence_handles: Read<'s, AssetSpawnsSequenceHandles>,

    /// `AssetCharacterCtsHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_cts_handles: Read<'s, AssetCharacterCtsHandles>,

    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Read<'s, AssetMapBounds>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Read<'s, AssetMargins>,
    /// `AssetLayerPositions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_layer_positions: Read<'s, AssetLayerPositions>,
}
