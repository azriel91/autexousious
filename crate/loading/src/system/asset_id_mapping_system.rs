use std::{iter::FromIterator, str::FromStr};

use amethyst::assets::ProgressCounter;
use asset_loading::ASSETS_DEFAULT_DIR;
use asset_model::{
    config::{AssetSlugBuilder, AssetType},
    loaded::AssetId,
};
use loading_model::loaded::LoadStage;
use log::debug;
use object_model::config::{GameObjectFrame, GameObjectSequence, ObjectDefinition};
use object_type::ObjectType;
use sequence_model::{config::SequenceNameString, loaded::SequenceIdMappings};
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequenceName;
use state_registry::StateId;
use ui_model::config::UiDefinition;

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem, DefinitionLoadingResourcesRead,
    IdMappingResources,
};

/// Maps asset sequence name strings to IDs.
pub type AssetIdMappingSystem = AssetPartLoadingSystem<AssetIdMapper>;

/// `AssetIdMapper`.
#[derive(Debug)]
pub struct AssetIdMapper;

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
                    map_definition_assets,
                    ui_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    asset_map_definition_handle,
                    asset_ui_definition_handle,
                    ..
                },
            asset_sequence_id_mappings_sprite,
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
            AssetType::Map => {
                let map_definition = asset_map_definition_handle
                    .get(asset_id)
                    .and_then(|map_definition_handle| {
                        map_definition_assets.get(map_definition_handle)
                    })
                    .expect("Expected `MapDefinition` to be loaded.");

                let sequence_id_mappings =
                    SequenceIdMappings::from_iter(map_definition.background.layers.keys().map(
                        |sequence_string| {
                            SequenceNameString::<SpriteSequenceName>::from_str(&sequence_string)
                                .expect("Expected `SequenceNameString::from_str` to succeed.")
                        },
                    ));

                asset_sequence_id_mappings_sprite.insert(asset_id, sequence_id_mappings);
            }
            AssetType::Ui => {
                // For UI, sequences exist in both `BackgroundDefinition` and `UiDefinition`.
                // However, we cannot simply merge both `sequence_id_mappings` because we would also
                // need to merge the sequences when being loaded, which is not possible --
                // `BackgroundDefinition` uses `SpriteSequence`, and `UiDefinition` uses
                // `UiSequence`.
                //
                // In addition, different UIs need to reference the `ControlSettings` UI's
                // configuration to display the mini control buttons display. The relevant
                // sequence ID mappings are the ones from `UiDefinition`. So we choose to store
                // those against the `AssetId`.
                //
                // Perhaps we should store `SequenceIdMappings` per `Asset` file instead of per
                // `AssetId` (which is a grouping of `Asset`s).

                // let background_definition = asset_background_definition_handle.get(asset_id)
                //     .and_then(|background_definition_handle| {
                //         background_definition_assets.get(background_definition_handle)
                //     },
                // );
                // if let Some(background_definition) = background_definition {
                //     let sequence_id_mappings = SequenceIdMappings::from_iter(
                //         background_definition.layers.keys().map(|sequence_string| {
                //             SequenceNameString::from_str(sequence_string)
                //                 .expect("Expected `SequenceNameString::from_str` to succeed.")
                //         }),
                //     );
                // }

                let ui_definition =
                    asset_ui_definition_handle
                        .get(asset_id)
                        .and_then(|ui_definition_handle| {
                            ui_definition_assets.get(ui_definition_handle)
                        });

                if let Some(UiDefinition { sequences, .. }) = ui_definition {
                    let sequence_id_mappings = SequenceIdMappings::from_iter(sequences.keys());
                    asset_sequence_id_mappings_sprite.insert(asset_id, sequence_id_mappings);
                }
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
            asset_id_mappings,
            asset_type_mappings,
            ..
        } = asset_loading_resources;
        let IdMappingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    ui_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    asset_ui_definition_handle,
                    ..
                },
            asset_sequence_id_mappings_sprite,
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
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
            AssetType::Map => asset_sequence_id_mappings_sprite.get(asset_id).is_some(),
            AssetType::Ui => {
                asset_ui_definition_handle
                    .get(asset_id)
                    .map(|ui_definition_handle| {
                        ui_definition_assets
                            .get(ui_definition_handle)
                            .expect("Expected `UiDefinition` to exist.")
                    })
                    .map(|ui_definition| {
                        // when there is an `AssetUiDefinition`, we check if the sequence ID
                        // mappings is populated.
                        let sequence_id_mappings_loaded =
                            asset_sequence_id_mappings_sprite.get(asset_id).is_some();

                        // In addition, for `UiDefinition`s that need to display mini control
                        // buttons, then we must also wait for the `control_settings` asset to have
                        // its sequence ID mappings populated.
                        let control_settings_sequence_id_mappings_loaded =
                            if ui_definition.display_control_buttons {
                                // Look up ControlSettings asset for mini control buttons display.
                                let asset_slug_control_settings = AssetSlugBuilder::default()
                                    .namespace(ASSETS_DEFAULT_DIR.to_string())
                                    .name(StateId::ControlSettings.to_string())
                                    .build()
                                    .expect("Expected control settings asset slug to be valid.");
                                let asset_id_control_settings = {
                                    let asset_id_control_settings =
                                        asset_id_mappings.id(&asset_slug_control_settings).copied();
                                    if asset_id_control_settings == Some(asset_id) {
                                        // If the current asset ID is the same as the control_settings `AssetId`, then we
                                        // return None -- we don't allow displaying the mini control buttons. This also
                                        // prevents waiting on our own asset ID to be loaded.
                                        None
                                    } else {
                                        asset_id_control_settings
                                    }
                                };

                                if let Some(asset_id_control_settings) = asset_id_control_settings {
                                    asset_sequence_id_mappings_sprite
                                        .get(asset_id_control_settings)
                                        .is_some()
                                } else {
                                    true
                                }
                            } else {
                                true
                            };

                        sequence_id_mappings_loaded && control_settings_sequence_id_mappings_loaded
                    })
                    .unwrap_or(true) // Default to true when there is no asset UI definition.
            }
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
            .flat_map(|game_obj_seq| game_obj_seq.object_sequence().sequence.frames.iter())
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
