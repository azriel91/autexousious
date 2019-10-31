use amethyst::{
    ecs::{Read, World, Write},
    shred::{ResourceId, SystemData},
};
use character_model::config::CharacterSequenceName;
use derivative::Derivative;
use energy_model::config::EnergySequenceName;
use sequence_model::loaded::AssetSequenceIdMappings;
use ui_model_spi::config::UiSequenceName;

use crate::DefinitionLoadingResourcesRead;

/// `IdMappingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct IdMappingResources<'s> {
    /// `DefinitionLoadingResourcesRead`.
    pub definition_loading_resources_read: DefinitionLoadingResourcesRead<'s>,
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Write<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetSequenceIdMappings<EnergySequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_energy: Write<'s, AssetSequenceIdMappings<EnergySequenceName>>,
    /// `AssetSequenceIdMappings<UiSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_ui: Write<'s, AssetSequenceIdMappings<UiSequenceName>>,
}

/// `IdMappingResourcesRead`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct IdMappingResourcesRead<'s> {
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Read<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetSequenceIdMappings<EnergySequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_energy: Read<'s, AssetSequenceIdMappings<EnergySequenceName>>,
    /// `AssetSequenceIdMappings<UiSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_ui: Read<'s, AssetSequenceIdMappings<UiSequenceName>>,
}
