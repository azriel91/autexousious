use std::iter::FromIterator;

use amethyst::{
    assets::ProgressCounter,
    ecs::{System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_model::{config::AssetType, loaded::AssetId};
use character_model::config::CharacterSequenceName;
use derivative::Derivative;
use derive_new::new;
use energy_model::config::EnergySequenceName;
use loading_model::loaded::{AssetLoadStage, LoadStage};
use log::debug;
use object_type::ObjectType;
use sequence_model::loaded::{AssetSequenceIdMappings, SequenceIdMappings};
use typename_derive::TypeName;

use crate::{AssetLoadingResources, DefinitionLoadingResources};

/// Maps asset sequence name strings to IDs.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetIdMappingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetIdMappingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Write<'s, AssetLoadStage>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// `DefinitionLoadingResources`.
    pub definition_loading_resources: DefinitionLoadingResources<'s>,
    /// `IdMappingResources`.
    pub id_mapping_resources: IdMappingResources<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct IdMappingResources<'s> {
    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Write<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetSequenceIdMappings<EnergySequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_energy: Write<'s, AssetSequenceIdMappings<EnergySequenceName>>,
}

impl<'s> System<'s> for AssetIdMappingSystem {
    type SystemData = AssetIdMappingSystemData<'s>;

    fn run(
        &mut self,
        AssetIdMappingSystemData {
            mut asset_load_stage,
            mut asset_loading_resources,
            definition_loading_resources,
            mut id_mapping_resources,
        }: Self::SystemData,
    ) {
        let capacity = asset_loading_resources.asset_id_mappings.capacity();
        let IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
        } = &mut id_mapping_resources;
        asset_sequence_id_mappings_character.set_capacity(capacity);
        asset_sequence_id_mappings_energy.set_capacity(capacity);

        asset_load_stage
            .iter_mut()
            .filter(|(_, load_stage)| **load_stage == LoadStage::DefinitionLoading)
            .for_each(|(asset_id, load_stage)| {
                if Self::definition_loaded(
                    &mut asset_loading_resources,
                    &definition_loading_resources,
                    asset_id,
                ) {
                    Self::id_map(
                        &mut asset_loading_resources,
                        &definition_loading_resources,
                        &mut id_mapping_resources,
                        asset_id,
                    );

                    *load_stage = LoadStage::IdMapping
                }
            });
    }
}

impl AssetIdMappingSystem {
    /// Returns whether the definition asset has been loaded.
    fn definition_loaded(
        AssetLoadingResources {
            asset_type_mappings,
            ..
        }: &mut AssetLoadingResources<'_>,
        DefinitionLoadingResources {
            character_definition_assets,
            energy_definition_assets,
            map_definition_assets,
            asset_character_definition_handle,
            asset_energy_definition_handle,
            asset_map_definition_handle,
        }: &DefinitionLoadingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        match asset_type {
            AssetType::Object(object_type) => match object_type {
                ObjectType::Character => {
                    let character_definition_handle = asset_character_definition_handle
                        .get(asset_id)
                        .expect("Expected `CharacterDefinitionHandle` to exist.");
                    character_definition_assets
                        .get(character_definition_handle)
                        .is_some()
                }
                ObjectType::Energy => {
                    let energy_definition_handle = asset_energy_definition_handle
                        .get(asset_id)
                        .expect("Expected `EnergyDefinitionHandle` to exist.");
                    energy_definition_assets
                        .get(energy_definition_handle)
                        .is_some()
                }
                ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
            },
            AssetType::Map => {
                let map_definition_handle = asset_map_definition_handle
                    .get(asset_id)
                    .expect("Expected `MapDefinitionHandle` to exist.");
                map_definition_assets.get(map_definition_handle).is_some()
            }
        }
    }

    /// Map's an asset's sequence IDs.
    fn id_map(
        AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            load_stage_progress_counters,
            ..
        }: &mut AssetLoadingResources,
        DefinitionLoadingResources {
            character_definition_assets,
            energy_definition_assets,
            asset_character_definition_handle,
            asset_energy_definition_handle,
            ..
        }: &DefinitionLoadingResources<'_>,
        IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
        }: &mut IdMappingResources<'_>,
        asset_id: AssetId,
    ) {
        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        debug!("Mapping IDs for asset `{}`", asset_slug);

        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let _progress_counter = load_stage_progress_counters
            .entry(LoadStage::IdMapping)
            .or_insert_with(ProgressCounter::new);

        match asset_type {
            AssetType::Object(object_type) => match object_type {
                ObjectType::Character => {
                    let character_definition = asset_character_definition_handle
                        .get(asset_id)
                        .and_then(|character_definition_handle| {
                            character_definition_assets.get(character_definition_handle)
                        })
                        .expect("Expected `CharacterDefinition` to be loaded.");

                    let sequence_id_mappings = SequenceIdMappings::from_iter(
                        character_definition.object_definition.sequences.keys(),
                    );
                    asset_sequence_id_mappings_character.insert(asset_id, sequence_id_mappings);
                }
                ObjectType::Energy => {
                    let energy_definition = asset_energy_definition_handle
                        .get(asset_id)
                        .and_then(|energy_definition_handle| {
                            energy_definition_assets.get(energy_definition_handle)
                        })
                        .expect("Expected `CharacterDefinition` to be loaded.");

                    let sequence_id_mappings = SequenceIdMappings::from_iter(
                        energy_definition.object_definition.sequences.keys(),
                    );
                    asset_sequence_id_mappings_energy.insert(asset_id, sequence_id_mappings);
                }
                ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
            },
            AssetType::Map => {}
        }
    }
}
