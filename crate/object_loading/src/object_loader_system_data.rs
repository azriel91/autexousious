use amethyst::{
    assets::{AssetStorage, Loader},
    audio::Source,
    ecs::{Read, ReadExpect, World},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::{AssetIdMappings, AssetTypeMappings};
use audio_model::loaded::SourceSequence;
use character_model::config::CharacterSequenceName;
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use derivative::Derivative;
use energy_model::config::EnergySequenceName;
use kinematic_model::loaded::ObjectAccelerationSequence;
use sequence_model::loaded::{AssetSequenceIdMappings, WaitSequence};
use spawn_model::loaded::{Spawns, SpawnsSequence};
use sprite_model::loaded::SpriteRenderSequence;

/// Resources needed to load an object.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectLoaderSystemData<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Read<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetSequenceIdMappings<EnergySequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_energy: Read<'s, AssetSequenceIdMappings<EnergySequenceName>>,
    /// `WaitSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `Source`s assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: Read<'s, AssetStorage<Source>>,
    /// `SourceSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub source_sequence_assets: Read<'s, AssetStorage<SourceSequence>>,
    /// `ObjectAccelerationSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub object_acceleration_sequence_assets: Read<'s, AssetStorage<ObjectAccelerationSequence>>,
    /// `SpriteRenderSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
    /// `BodySequence`s assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: Read<'s, AssetStorage<BodySequence>>,
    /// `InteractionsSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: Read<'s, AssetStorage<InteractionsSequence>>,
    /// `SpawnsSequence`s assets.
    #[derivative(Debug = "ignore")]
    pub spawns_sequence_assets: Read<'s, AssetStorage<SpawnsSequence>>,
    /// `Body` assets.
    #[derivative(Debug = "ignore")]
    pub body_assets: Read<'s, AssetStorage<Body>>,
    /// `Interactions` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: Read<'s, AssetStorage<Interactions>>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,
}
