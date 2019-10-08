use amethyst::{
    ecs::{Read, World},
    shred::{ResourceId, SystemData},
};
use character_model::config::CharacterSequenceName;
use derivative::Derivative;
use energy_model::config::EnergySequenceName;
use sequence_model::loaded::AssetSequenceIdMappings;

/// `IdMappingResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct IdMappingResources<'s> {
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Read<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetSequenceIdMappings<EnergySequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_energy: Read<'s, AssetSequenceIdMappings<EnergySequenceName>>,
}
