use amethyst::{
    assets::AssetStorage,
    ecs::{Read, World, Write},
    shred::{ResourceId, SystemData},
};
use background_model::{config::BackgroundDefinition, loaded::AssetBackgroundDefinitionHandle};
use character_model::{config::CharacterDefinition, loaded::AssetCharacterDefinitionHandle};
use derivative::Derivative;
use energy_model::{config::EnergyDefinition, loaded::AssetEnergyDefinitionHandle};
use map_model::{config::MapDefinition, loaded::AssetMapDefinitionHandle};
use ui_model::{config::UiDefinition, loaded::AssetUiDefinitionHandle};

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
    /// `BackgroundDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub background_definition_assets: Read<'s, AssetStorage<BackgroundDefinition>>,
    /// `UiDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub ui_definition_assets: Read<'s, AssetStorage<UiDefinition>>,
    /// `AssetCharacterDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_definition_handle: Write<'s, AssetCharacterDefinitionHandle>,
    /// `AssetEnergyDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_energy_definition_handle: Write<'s, AssetEnergyDefinitionHandle>,
    /// `AssetMapDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_definition_handle: Write<'s, AssetMapDefinitionHandle>,
    /// `AssetBackgroundDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_background_definition_handle: Write<'s, AssetBackgroundDefinitionHandle>,
    /// `AssetUiDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_ui_definition_handle: Write<'s, AssetUiDefinitionHandle>,
}

/// `DefinitionLoadingResourcesRead`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct DefinitionLoadingResourcesRead<'s> {
    /// `CharacterDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub character_definition_assets: Read<'s, AssetStorage<CharacterDefinition>>,
    /// `EnergyDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub energy_definition_assets: Read<'s, AssetStorage<EnergyDefinition>>,
    /// `MapDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub map_definition_assets: Read<'s, AssetStorage<MapDefinition>>,
    /// `BackgroundDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub background_definition_assets: Read<'s, AssetStorage<BackgroundDefinition>>,
    /// `UiDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub ui_definition_assets: Read<'s, AssetStorage<UiDefinition>>,
    /// `AssetCharacterDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_definition_handle: Read<'s, AssetCharacterDefinitionHandle>,
    /// `AssetEnergyDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_energy_definition_handle: Read<'s, AssetEnergyDefinitionHandle>,
    /// `AssetMapDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_definition_handle: Read<'s, AssetMapDefinitionHandle>,
    /// `AssetBackgroundDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_background_definition_handle: Read<'s, AssetBackgroundDefinitionHandle>,
    /// `AssetUiDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_ui_definition_handle: Read<'s, AssetUiDefinitionHandle>,
}
