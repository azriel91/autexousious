use amethyst::{
    assets::AssetStorage,
    ecs::{Read, World},
    shred::{ResourceId, SystemData},
};
use character_model::{config::CharacterDefinition, loaded::AssetCharacterDefinitionHandle};
use derivative::Derivative;
use energy_model::{config::EnergyDefinition, loaded::AssetEnergyDefinitionHandle};
use map_model::{config::MapDefinition, loaded::AssetMapDefinitionHandle};

/// `DefinitionLoadingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct DefinitionLoadingResources<'s> {
    /// `CharacterDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub character_definition_assets: Read<'s, AssetStorage<CharacterDefinition>>,
    /// `EnergyDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub energy_definition_assets: Read<'s, AssetStorage<EnergyDefinition>>,
    /// `MapDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub map_definition_assets: Read<'s, AssetStorage<MapDefinition>>,
    /// `AssetCharacterDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_definition_handle: Read<'s, AssetCharacterDefinitionHandle>,
    /// `AssetEnergyDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_energy_definition_handle: Read<'s, AssetEnergyDefinitionHandle>,
    /// `AssetMapDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_definition_handle: Read<'s, AssetMapDefinitionHandle>,
}
