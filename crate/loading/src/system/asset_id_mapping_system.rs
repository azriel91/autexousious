use std::{iter::FromIterator, str::FromStr};

use amethyst::assets::ProgressCounter;
use asset_model::{config::AssetType, loaded::AssetId};
use loading_model::loaded::LoadStage;
use log::debug;
use object_model::config::{GameObjectFrame, GameObjectSequence, ObjectDefinition};
use object_type::ObjectType;
use sequence_model::{
    config::{SequenceName, SequenceNameString},
    loaded::SequenceIdMappings,
};
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequence;
use typename_derive::TypeName;

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem, DefinitionLoadingResourcesRead,
    IdMappingResources,
};

/// Maps asset sequence name strings to IDs.
pub type AssetIdMappingSystem = AssetPartLoadingSystem<AssetIdMapper>;

/// `AssetIdMapper`.
#[derive(Debug, TypeName)]
pub struct AssetIdMapper;

impl AssetIdMapper {
    fn layer_to_sequence_name_string<'layers, SeqName>(
        layers: &'layers [SpriteSequence],
    ) -> impl Iterator<Item = SequenceNameString<SeqName>> + 'layers
    where
        SeqName: SequenceName,
    {
        layers.iter().enumerate().map(|(layer_index, _)| {
            SequenceNameString::<SeqName>::from_str(&format!("background_layer_{}", layer_index))
                .expect("Expected sequence name string to be valid.")
        })
    }
}

impl<'s> AssetPartLoader<'s> for AssetIdMapper {
    const LOAD_STAGE: LoadStage = LoadStage::IdMapping;
    type SystemData = IdMappingResources<'s>;

    fn preprocess(
        AssetLoadingResources {
            asset_id_mappings, ..
        }: &mut AssetLoadingResources,
        IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
            ..
        }: &mut IdMappingResources<'_>,
    ) {
        let capacity = asset_id_mappings.capacity();
        asset_sequence_id_mappings_character.set_capacity(capacity);
        asset_sequence_id_mappings_energy.set_capacity(capacity);
    }

    /// Map's an asset's sequence IDs.
    fn process(
        AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            load_stage_progress_counters,
            ..
        }: &mut AssetLoadingResources,
        IdMappingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    background_definition_assets,
                    ui_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    asset_background_definition_handle,
                    asset_ui_definition_handle,
                    ..
                },
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
            asset_sequence_id_mappings_ui,
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
            AssetType::Ui => {
                let background_definition = asset_background_definition_handle
                    .get(asset_id)
                    .and_then(|background_definition_handle| {
                        background_definition_assets.get(background_definition_handle)
                    });

                let ui_definition =
                    asset_ui_definition_handle
                        .get(asset_id)
                        .and_then(|ui_definition_handle| {
                            ui_definition_assets.get(ui_definition_handle)
                        });

                let sequence_id_mappings = match (background_definition, ui_definition) {
                    (None, None) => SequenceIdMappings::new(),
                    (None, Some(ui_definition)) => {
                        SequenceIdMappings::from_iter(ui_definition.sequences.keys())
                    }
                    (Some(background_definition), None) => SequenceIdMappings::from_iter(
                        Self::layer_to_sequence_name_string(&background_definition.layers),
                    ),
                    (Some(background_definition), Some(ui_definition)) => {
                        SequenceIdMappings::from_iter(
                            Self::layer_to_sequence_name_string(&background_definition.layers)
                                .chain(ui_definition.sequences.keys().cloned()),
                        )
                    }
                };

                asset_sequence_id_mappings_ui.insert(asset_id, sequence_id_mappings);
            }
        }
    }

    /// Returns whether ID mappings has been completed.
    fn is_complete(
        asset_loading_resources: &AssetLoadingResources,
        id_mapping_resources: &IdMappingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        let AssetLoadingResources {
            asset_type_mappings,
            ..
        } = asset_loading_resources;
        let IdMappingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    ..
                },
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
            asset_sequence_id_mappings_ui,
        } = id_mapping_resources;

        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        match asset_type {
            AssetType::Object(object_type) => match object_type {
                ObjectType::Character => {
                    let id_mappings_self =
                        asset_sequence_id_mappings_character.get(asset_id).is_some();
                    let spawn_id_mappings_exist = {
                        let character_definition = asset_character_definition_handle
                            .get(asset_id)
                            .and_then(|character_definition_handle| {
                                character_definition_assets.get(character_definition_handle)
                            })
                            .expect("Expected `CharacterDefinition` to be loaded.");

                        Self::spawn_object_sequence_id_mappings_loaded(
                            asset_loading_resources,
                            id_mapping_resources,
                            &character_definition.object_definition,
                        )
                    };

                    id_mappings_self && spawn_id_mappings_exist
                }
                ObjectType::Energy => {
                    let id_mappings_self =
                        asset_sequence_id_mappings_energy.get(asset_id).is_some();
                    let spawn_id_mappings_exist = {
                        let energy_definition = asset_energy_definition_handle
                            .get(asset_id)
                            .and_then(|energy_definition_handle| {
                                energy_definition_assets.get(energy_definition_handle)
                            })
                            .expect("Expected `EnergyDefinition` to be loaded.");

                        Self::spawn_object_sequence_id_mappings_loaded(
                            asset_loading_resources,
                            id_mapping_resources,
                            &energy_definition.object_definition,
                        )
                    };

                    id_mappings_self && spawn_id_mappings_exist
                }
                ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
            },
            AssetType::Map => true,
            AssetType::Ui => asset_sequence_id_mappings_ui.get(asset_id).is_some(),
        }
    }
}

impl AssetIdMapper {
    fn spawn_object_sequence_id_mappings_loaded<ObjSeq>(
        AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            ..
        }: &AssetLoadingResources<'_>,
        IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
            ..
        }: &IdMappingResources<'_>,
        object_definition: &ObjectDefinition<ObjSeq>,
    ) -> bool
    where
        ObjSeq: GameObjectSequence,
        ObjSeq::SequenceName: for<'des> Deserialize<'des> + Serialize,
    {
        object_definition
            .sequences
            .values()
            .flat_map(|game_obj_seq| game_obj_seq.object_sequence().frames.iter())
            .flat_map(|frame| frame.object_frame().spawns.iter())
            .try_fold((), |_, spawn| {
                // Check if sequence ID mappings exist for `spawn.object`.
                let spawn_asset_slug = &spawn.object;
                let spawn_asset_id = asset_id_mappings
                    .id(spawn_asset_slug)
                    .copied()
                    .unwrap_or_else(|| panic!("Asset ID not found for `{}`.", spawn_asset_slug));
                let spawn_asset_type = asset_type_mappings
                    .get(spawn_asset_id)
                    .expect("Expected `AssetType` mapping to exist.");

                let spawn_id_mappings_exist = match spawn_asset_type {
                    AssetType::Object(spawn_object_type) => match spawn_object_type {
                        ObjectType::Character => asset_sequence_id_mappings_character
                            .get(spawn_asset_id)
                            .is_some(),
                        ObjectType::Energy => asset_sequence_id_mappings_energy
                            .get(spawn_asset_id)
                            .is_some(),
                        ObjectType::TestObject => {
                            panic!("Spawning `TestObject`s is not supported.")
                        }
                    },
                    AssetType::Map => panic!("Spawning `Map`s is not supported."),
                    AssetType::Ui => panic!("Spawning `Ui`s is not supported."),
                };
                if spawn_id_mappings_exist {
                    Ok(())
                } else {
                    Err(())
                }
            })
            .is_ok()
    }
}
