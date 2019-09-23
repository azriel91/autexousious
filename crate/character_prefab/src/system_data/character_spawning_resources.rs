use amethyst::{
    assets::AssetStorage,
    ecs::{Read, World},
    shred::{ResourceId, SystemData},
};
use character_model::{
    config::{CharacterDefinition, CharacterSequenceName},
    loaded::AssetCharacterDefinitionHandle,
};
use derivative::Derivative;
use sequence_model::loaded::AssetSequenceIdMappings;

/// Resources used to spawn character entities.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSpawningResources<'s> {
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Read<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetCharacterDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_definition_handle: Read<'s, AssetCharacterDefinitionHandle>,
    /// `CharacterDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub character_definition_assets: Read<'s, AssetStorage<CharacterDefinition>>,
}
