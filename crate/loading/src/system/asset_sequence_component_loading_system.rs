use std::str::FromStr;

use amethyst::{
    ecs::{Builder, Entity, WorldExt},
    renderer::SpriteRender,
};
use asset_model::{config::AssetType, loaded::AssetId};
use audio_model::loaded::SourceSequenceHandles;
use character_loading::{CtsLoader, CtsLoaderParams, CHARACTER_TRANSITIONS_DEFAULT};
use character_model::{
    config::CharacterSequence,
    loaded::{CharacterCtsHandle, CharacterCtsHandles},
};
use collision_model::loaded::{BodySequenceHandles, InteractionsSequenceHandles};
use control_settings_loading::KeyboardUiGen;
use energy_model::config::EnergySequence;
use game_mode_selection_model::GameModeIndex;
use kinematic_loading::PositionInitsLoader;
use kinematic_model::{
    config::PositionInit,
    loaded::{ObjectAccelerationSequenceHandles, PositionInits},
};
use loading_model::loaded::LoadStage;
use log::debug;
use map_model::{config::MapBounds, loaded::Margins};
use object_loading::{ObjectLoader, ObjectLoaderParams};
use object_model::loaded::Object;
use object_type::ObjectType;
use sequence_loading::{
    SequenceEndTransitionMapper, SequenceEndTransitionsLoader, WaitSequenceHandlesLoader,
    WaitSequenceLoader,
};
use sequence_model::{
    config::SequenceNameString,
    loaded::{SequenceEndTransitions, WaitSequenceHandles},
};
use spawn_model::loaded::SpawnsSequenceHandles;
use sprite_loading::{
    ScaleSequenceHandlesLoader, ScaleSequenceLoader, SpriteRenderSequenceHandlesLoader,
    SpriteRenderSequenceLoader, TintSequenceHandlesLoader, TintSequenceLoader,
};
use sprite_model::{
    config::SpriteFrame,
    loaded::{ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles},
};
use typename_derive::TypeName;
use ui_label_loading::{UiLabelsLoader, UiSpriteLabelsLoader};
use ui_label_model::{
    config::{self, UiLabel, UiLabels},
    loaded::{UiSpriteLabel, UiSpriteLabels},
};
use ui_menu_item_loading::UiMenuItemsLoader;
use ui_menu_item_model::loaded::UiMenuItem;
use ui_model::config::{UiDefinition, UiType};

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem, DefinitionLoadingResourcesRead,
    IdMappingResourcesRead, SequenceComponentLoadingResources, TextureLoadingResourcesRead,
};

/// Loads asset sequence components.
pub type AssetSequenceComponentLoadingSystem = AssetPartLoadingSystem<AssetSequenceComponentLoader>;

/// `AssetSequenceComponentLoader`.
#[derive(Debug, TypeName)]
pub struct AssetSequenceComponentLoader;

impl<'s> AssetPartLoader<'s> for AssetSequenceComponentLoader {
    const LOAD_STAGE: LoadStage = LoadStage::SequenceComponentLoading;
    type SystemData = SequenceComponentLoadingResources<'s>;

    fn preprocess(
        AssetLoadingResources {
            asset_id_mappings, ..
        }: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            asset_world,
            asset_sequence_end_transitions,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            ..
        }: &mut SequenceComponentLoadingResources<'_>,
    ) {
        asset_world.register::<SequenceEndTransitions>();
        asset_world.register::<WaitSequenceHandles>();
        asset_world.register::<SourceSequenceHandles>();
        asset_world.register::<ObjectAccelerationSequenceHandles>();
        asset_world.register::<SpriteRenderSequenceHandles>();
        asset_world.register::<BodySequenceHandles>();
        asset_world.register::<InteractionsSequenceHandles>();
        asset_world.register::<SpawnsSequenceHandles>();
        asset_world.register::<CharacterCtsHandles>();
        asset_world.register::<PositionInit>();
        asset_world.register::<TintSequenceHandles>();
        asset_world.register::<ScaleSequenceHandles>();
        asset_world.register::<MapBounds>();
        asset_world.register::<Margins>();
        asset_world.register::<UiLabel>();
        asset_world.register::<UiSpriteLabel>();
        asset_world.register::<UiMenuItem<GameModeIndex>>();

        let capacity = asset_id_mappings.capacity();
        asset_sequence_end_transitions.set_capacity(capacity);
        asset_wait_sequence_handles.set_capacity(capacity);
        asset_source_sequence_handles.set_capacity(capacity);
        asset_object_acceleration_sequence_handles.set_capacity(capacity);
        asset_sprite_render_sequence_handles.set_capacity(capacity);
        asset_body_sequence_handles.set_capacity(capacity);
        asset_interactions_sequence_handles.set_capacity(capacity);
        asset_spawns_sequence_handles.set_capacity(capacity);
    }

    fn process(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    map_definition_assets,
                    background_definition_assets,
                    ui_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    asset_map_definition_handle,
                    asset_background_definition_handle,
                    asset_ui_definition_handle,
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    asset_sequence_id_mappings_sprite,
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
                    ..
                },
            texture_loading_resources_read:
                TextureLoadingResourcesRead {
                    asset_sprite_sheet_handles,
                    ..
                },
            asset_world,
            input_config,
            source_assets,
            body_assets,
            interactions_assets,
            spawns_assets,
            wait_sequence_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            character_control_transitions_assets,
            character_cts_assets,
            tint_sequence_assets,
            scale_sequence_assets,
            asset_sequence_end_transitions,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            asset_character_cts_handles,
            asset_position_inits,
            asset_tint_sequence_handles,
            asset_scale_sequence_handles,
            asset_map_bounds,
            asset_margins,
            asset_ui_labels,
            asset_ui_sprite_labels,
            asset_ui_menu_items,
        }: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            loader,
            ..
        } = asset_loading_resources;

        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        debug!("Loading `{}` sequence components.", asset_slug);

        let wait_sequence_loader = WaitSequenceLoader {
            loader,
            wait_sequence_assets,
        };
        let mut wait_sequence_handles_loader = WaitSequenceHandlesLoader {
            wait_sequence_loader,
            asset_wait_sequence_handles,
        };
        let tint_sequence_loader = TintSequenceLoader {
            loader,
            tint_sequence_assets,
        };
        let mut tint_sequence_handles_loader = TintSequenceHandlesLoader {
            tint_sequence_loader,
            asset_tint_sequence_handles,
        };
        let scale_sequence_loader = ScaleSequenceLoader {
            loader,
            scale_sequence_assets,
        };
        let mut scale_sequence_handles_loader = ScaleSequenceHandlesLoader {
            scale_sequence_loader,
            asset_scale_sequence_handles,
        };
        let sprite_render_sequence_loader = SpriteRenderSequenceLoader {
            loader,
            sprite_render_sequence_assets,
        };
        let mut sprite_render_sequence_handles_loader = SpriteRenderSequenceHandlesLoader {
            sprite_render_sequence_loader,
            asset_sprite_render_sequence_handles,
        };

        let mut position_inits_loader = PositionInitsLoader {
            asset_position_inits,
        };
        let sequence_end_transition_mapper_ui = SequenceEndTransitionMapper {
            asset_sequence_id_mappings: asset_sequence_id_mappings_sprite,
        };
        let sequence_end_transition_mapper_sprite = SequenceEndTransitionMapper {
            asset_sequence_id_mappings: asset_sequence_id_mappings_sprite,
        };

        // let mut ui_labels_loader = UiLabelsLoader { asset_ui_labels };
        let mut ui_sprite_labels_loader = UiSpriteLabelsLoader {
            asset_id_mappings,
            asset_sequence_id_mappings_sprite,
            asset_ui_sprite_labels,
        };
        let mut ui_menu_items_loader = UiMenuItemsLoader {
            asset_id_mappings,
            asset_sequence_id_mappings_sprite,
            asset_ui_menu_items,
        };

        let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);
        match asset_type {
            AssetType::Object(object_type) => {
                let mut item_entity_builder = asset_world.create_entity();

                let sprite_sheet_handles = sprite_sheet_handles
                    .expect("Expected `SpriteSheetHandles` to exist for object.");
                let object_loader_params = ObjectLoaderParams {
                    loader,
                    asset_id_mappings,
                    asset_type_mappings,
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
                    wait_sequence_assets,
                    source_assets,
                    source_sequence_assets,
                    object_acceleration_sequence_assets,
                    sprite_render_sequence_assets,
                    body_sequence_assets,
                    interactions_sequence_assets,
                    spawns_sequence_assets,
                    body_assets,
                    interactions_assets,
                    spawns_assets,
                    sprite_sheet_handles,
                };

                let object = match object_type {
                    ObjectType::Character => {
                        let character_definition = asset_character_definition_handle
                            .get(asset_id)
                            .and_then(|character_definition_handle| {
                                character_definition_assets.get(character_definition_handle)
                            })
                            .expect("Expected `CharacterDefinition` to be loaded.");

                        let sequence_id_mappings = asset_sequence_id_mappings_character
                            .get(asset_id)
                            .expect("Expected `SequenceIdMapping` to be loaded.");

                        let cts_loader_params = CtsLoaderParams {
                            loader: &*loader,
                            character_control_transitions_assets:
                                &*character_control_transitions_assets,
                            character_cts_assets: &*character_cts_assets,
                        };

                        let character_cts_handles = {
                            let character_cts_handles = character_definition
                                .object_definition
                                .sequences
                                .iter()
                                .map(|(sequence_id, sequence)| {
                                    let sequence_default = CHARACTER_TRANSITIONS_DEFAULT
                                        .object_definition
                                        .sequences
                                        .get(sequence_id);

                                    CtsLoader::load(
                                        &cts_loader_params,
                                        sequence_id_mappings,
                                        sequence_default,
                                        sequence,
                                    )
                                })
                                .collect::<Vec<CharacterCtsHandle>>();
                            CharacterCtsHandles::new(character_cts_handles)
                        };
                        asset_character_cts_handles.insert(asset_id, character_cts_handles.clone());

                        item_entity_builder = item_entity_builder.with(character_cts_handles);

                        ObjectLoader::load::<CharacterSequence>(
                            object_loader_params,
                            &character_definition.object_definition,
                        )
                    }
                    ObjectType::Energy => {
                        let energy_definition = asset_energy_definition_handle
                            .get(asset_id)
                            .and_then(|energy_definition_handle| {
                                energy_definition_assets.get(energy_definition_handle)
                            })
                            .expect("Expected `EnergyDefinition` to be loaded.");

                        ObjectLoader::load::<EnergySequence>(
                            object_loader_params,
                            &energy_definition.object_definition,
                        )
                    }
                    ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
                };
                let Object {
                    sequence_end_transitions,
                    wait_sequence_handles,
                    source_sequence_handles,
                    object_acceleration_sequence_handles,
                    sprite_render_sequence_handles,
                    body_sequence_handles,
                    interactions_sequence_handles,
                    spawns_sequence_handles,
                } = object;

                asset_sequence_end_transitions.insert(asset_id, sequence_end_transitions.clone());
                asset_wait_sequence_handles.insert(asset_id, wait_sequence_handles.clone());
                asset_source_sequence_handles.insert(asset_id, source_sequence_handles.clone());
                asset_object_acceleration_sequence_handles
                    .insert(asset_id, object_acceleration_sequence_handles.clone());
                asset_sprite_render_sequence_handles
                    .insert(asset_id, sprite_render_sequence_handles.clone());
                asset_body_sequence_handles.insert(asset_id, body_sequence_handles.clone());
                asset_interactions_sequence_handles
                    .insert(asset_id, interactions_sequence_handles.clone());
                asset_spawns_sequence_handles.insert(asset_id, spawns_sequence_handles.clone());

                let _item_entity = item_entity_builder
                    .with(sequence_end_transitions)
                    .with(wait_sequence_handles)
                    .with(source_sequence_handles)
                    .with(object_acceleration_sequence_handles)
                    .with(sprite_render_sequence_handles)
                    .with(body_sequence_handles)
                    .with(interactions_sequence_handles)
                    .with(spawns_sequence_handles)
                    .build();
            }
            AssetType::Map => {
                let map_definition = asset_map_definition_handle
                    .get(asset_id)
                    .and_then(|map_definition_handle| {
                        map_definition_assets.get(map_definition_handle)
                    })
                    .expect("Expected `MapDefinition` to be loaded.");

                let mut sequence_end_transitions_loader = SequenceEndTransitionsLoader {
                    sequence_end_transition_mapper: sequence_end_transition_mapper_sprite,
                    asset_sequence_end_transitions,
                };

                let background_definition = &map_definition.background;

                let position_inits =
                    PositionInitsLoader::items_to_datas(background_definition.layers.values());
                let ui_sprite_labels = {
                    let layers_as_ui_sprite_label =
                        background_definition
                            .layers
                            .iter()
                            .map(|(sequence_string, layer)| {
                                let sequence_name_string = SequenceNameString::from_str(
                                    sequence_string,
                                )
                                .expect("Expected `SequenceNameString::from_str` to succeed.");
                                config::UiSpriteLabel::new(layer.position, sequence_name_string)
                            });
                    ui_sprite_labels_loader.items_to_datas(layers_as_ui_sprite_label, asset_id)
                };
                let sequence_end_transitions = sequence_end_transitions_loader
                    .items_to_datas(background_definition.layers.values(), asset_id);
                let wait_sequence_handles = wait_sequence_handles_loader
                    .items_to_datas(background_definition.layers.values(), |layer| {
                        layer.sequence.frames.iter()
                    });
                let tint_sequence_handles = tint_sequence_handles_loader
                    .items_to_datas(background_definition.layers.values(), |layer| {
                        layer.sequence.frames.iter()
                    });
                let scale_sequence_handles = scale_sequence_handles_loader
                    .items_to_datas(background_definition.layers.values(), |layer| {
                        layer.sequence.frames.iter()
                    });
                let sprite_render_sequence_handles =
                    sprite_sheet_handles.map(|sprite_sheet_handles| {
                        sprite_render_sequence_handles_loader.items_to_datas(
                            background_definition.layers.values(),
                            |layer| layer.sequence.frames.iter(),
                            sprite_sheet_handles,
                        )
                    });

                let _item_entities = position_inits
                    .0
                    .into_iter()
                    .zip(ui_sprite_labels.0.into_iter())
                    .map(|(position_init, ui_sprite_label)| {
                        let mut item_entity_builder = asset_world
                            .create_entity()
                            .with(position_init)
                            .with(ui_sprite_label)
                            .with(sequence_end_transitions.clone())
                            .with(wait_sequence_handles.clone())
                            .with(tint_sequence_handles.clone())
                            .with(scale_sequence_handles.clone());

                        if let Some(sprite_render_sequence_handles) =
                            sprite_render_sequence_handles.clone()
                        {
                            item_entity_builder =
                                item_entity_builder.with(sprite_render_sequence_handles);
                        }

                        item_entity_builder.build()
                    })
                    .collect::<Vec<Entity>>();

                // TODO: delete this block.
                if let Some(sprite_sheet_handles) = sprite_sheet_handles {
                    wait_sequence_handles_loader.load(
                        background_definition.layers.values(),
                        |layer| layer.sequence.frames.iter(),
                        asset_id,
                    );
                    sprite_render_sequence_handles_loader.load(
                        background_definition.layers.values(),
                        |layer| layer.sequence.frames.iter(),
                        asset_id,
                        sprite_sheet_handles,
                    );
                    tint_sequence_handles_loader.load(
                        background_definition.layers.values(),
                        |layer| layer.sequence.frames.iter(),
                        asset_id,
                    );
                    scale_sequence_handles_loader.load(
                        background_definition.layers.values(),
                        |layer| layer.sequence.frames.iter(),
                        asset_id,
                    );
                    position_inits_loader.load(background_definition.layers.values(), asset_id);

                    let layers_as_ui_sprite_label =
                        background_definition
                            .layers
                            .iter()
                            .map(|(sequence_string, layer)| {
                                let sequence_name_string = SequenceNameString::from_str(
                                    sequence_string,
                                )
                                .expect("Expected `SequenceNameString::from_str` to succeed.");
                                config::UiSpriteLabel::new(layer.position, sequence_name_string)
                            });
                    ui_sprite_labels_loader.load(layers_as_ui_sprite_label, asset_id);
                }

                sequence_end_transitions_loader
                    .load(background_definition.layers.values(), asset_id);

                let map_bounds = map_definition.header.bounds;
                asset_map_bounds.insert(asset_id, map_bounds);

                let margins = Margins::from(map_bounds);
                asset_margins.insert(asset_id, margins);
            }
            AssetType::Ui => {
                let background_definition = asset_background_definition_handle
                    .get(asset_id)
                    .and_then(|background_definition_handle| {
                        background_definition_assets.get(background_definition_handle)
                    });

                let mut ui_definition = asset_ui_definition_handle
                    .get(asset_id)
                    .and_then(|ui_definition_handle| ui_definition_assets.get(ui_definition_handle))
                    .cloned();

                let mut sequence_end_transitions_loader = SequenceEndTransitionsLoader {
                    sequence_end_transition_mapper: sequence_end_transition_mapper_sprite,
                    asset_sequence_end_transitions,
                };

                // Hack: Update `PositionInit`s for `UiDefinition.button`s
                if let Some(ui_definition) = ui_definition.as_mut() {
                    ui_definition.buttons.iter_mut().for_each(|ui_button| {
                        ui_button.label.position += ui_button.position;
                        ui_button.sprite.position += ui_button.position;
                    });
                }
                // End hack

                let keyboard_ui_sprite_labels = if let Some(UiDefinition {
                    ui_type: UiType::ControlSettings(control_settings),
                    sequences,
                    ..
                }) = ui_definition.as_mut()
                {
                    Some(KeyboardUiGen::generate(
                        &control_settings.keyboard,
                        &input_config,
                        sequences,
                    ))
                } else {
                    None
                };

                let ui_definition = ui_definition.as_ref();

                // Load item entities object-wise.
                if let Some(background_definition) = background_definition {
                    let position_inits =
                        PositionInitsLoader::items_to_datas(background_definition.layers.values());
                    let ui_sprite_labels = {
                        let layers_as_ui_sprite_label =
                            background_definition
                                .layers
                                .iter()
                                .map(|(sequence_string, layer)| {
                                    let sequence_name_string = SequenceNameString::from_str(
                                        sequence_string,
                                    )
                                    .expect("Expected `SequenceNameString::from_str` to succeed.");
                                    config::UiSpriteLabel::new(layer.position, sequence_name_string)
                                });
                        ui_sprite_labels_loader.items_to_datas(layers_as_ui_sprite_label, asset_id)
                    };
                    let sequence_end_transitions = sequence_end_transitions_loader
                        .items_to_datas(background_definition.layers.values(), asset_id);
                    let wait_sequence_handles = wait_sequence_handles_loader
                        .items_to_datas(background_definition.layers.values(), |layer| {
                            layer.sequence.frames.iter()
                        });
                    let tint_sequence_handles = tint_sequence_handles_loader
                        .items_to_datas(background_definition.layers.values(), |layer| {
                            layer.sequence.frames.iter()
                        });
                    let scale_sequence_handles = scale_sequence_handles_loader
                        .items_to_datas(background_definition.layers.values(), |layer| {
                            layer.sequence.frames.iter()
                        });
                    let sprite_render_sequence_handles =
                        sprite_sheet_handles.map(|sprite_sheet_handles| {
                            sprite_render_sequence_handles_loader.items_to_datas(
                                background_definition.layers.values(),
                                |layer| layer.sequence.frames.iter(),
                                sprite_sheet_handles,
                            )
                        });

                    let _item_entities = position_inits
                        .0
                        .into_iter()
                        .zip(ui_sprite_labels.0.into_iter())
                        .map(|(position_init, ui_sprite_label)| {
                            let mut item_entity_builder = asset_world
                                .create_entity()
                                .with(position_init)
                                .with(ui_sprite_label)
                                .with(sequence_end_transitions.clone())
                                .with(wait_sequence_handles.clone())
                                .with(tint_sequence_handles.clone())
                                .with(scale_sequence_handles.clone());

                            if let Some(sprite_render_sequence_handles) =
                                sprite_render_sequence_handles.clone()
                            {
                                item_entity_builder =
                                    item_entity_builder.with(sprite_render_sequence_handles);
                            }

                            item_entity_builder.build()
                        })
                        .collect::<Vec<Entity>>();
                }
                if let Some(ui_definition) = ui_definition {
                    let UiDefinition {
                        ui_type, sequences, ..
                    } = ui_definition;

                    let sequence_end_transitions = sequence_end_transitions_loader
                        .items_to_datas(sequences.values(), asset_id);
                    let wait_sequence_handles = wait_sequence_handles_loader
                        .items_to_datas(sequences.values(), |sequence| sequence.frames.iter());
                    let tint_sequence_handles = tint_sequence_handles_loader
                        .items_to_datas(sequences.values(), |sequence| sequence.frames.iter());
                    let scale_sequence_handles = scale_sequence_handles_loader
                        .items_to_datas(sequences.values(), |sequence| sequence.frames.iter());
                    let sprite_render_sequence_handles =
                        sprite_sheet_handles.map(|sprite_sheet_handles| {
                            sprite_render_sequence_handles_loader.items_to_datas(
                                sequences.values(),
                                |sequence| sequence.frames.iter(),
                                sprite_sheet_handles,
                            )
                        });

                    match ui_type {
                        UiType::Menu(ui_menu_items_cfg) => {
                            let position_inits =
                                PositionInitsLoader::items_to_datas(ui_menu_items_cfg.iter());
                            let ui_menu_items = ui_menu_items_loader
                                .items_to_datas(ui_menu_items_cfg.iter(), asset_id);
                            let ui_labels =
                                UiLabelsLoader::items_to_datas(ui_menu_items_cfg.iter());
                            let ui_sprite_labels = ui_sprite_labels_loader
                                .items_to_datas(ui_menu_items_cfg.iter(), asset_id);

                            let _item_entities = position_inits
                                .0
                                .into_iter()
                                .zip(ui_sprite_labels.0.into_iter())
                                .zip(ui_menu_items.0.into_iter())
                                .zip(ui_labels.0.into_iter())
                                .map(
                                    |(
                                        ((position_init, ui_sprite_label), ui_menu_item),
                                        ui_label,
                                    )| {
                                        let mut item_entity_builder = asset_world
                                            .create_entity()
                                            .with(position_init)
                                            .with(ui_sprite_label)
                                            .with(ui_menu_item)
                                            .with(ui_label)
                                            .with(sequence_end_transitions.clone())
                                            .with(wait_sequence_handles.clone())
                                            .with(tint_sequence_handles.clone())
                                            .with(scale_sequence_handles.clone());

                                        if let Some(sprite_render_sequence_handles) =
                                            sprite_render_sequence_handles.clone()
                                        {
                                            item_entity_builder = item_entity_builder
                                                .with(sprite_render_sequence_handles);
                                        }

                                        item_entity_builder.build()
                                    },
                                )
                                .collect::<Vec<Entity>>();
                        }
                        UiType::ControlSettings(control_settings) => {
                            let keyboard_ui_sprite_labels = keyboard_ui_sprite_labels
                                .as_ref()
                                .expect("Expected `keyboard_ui_sprite_labels` to exist.");
                            let position_inits = PositionInitsLoader::items_to_datas(
                                keyboard_ui_sprite_labels.iter(),
                            );
                            let ui_sprite_labels = ui_sprite_labels_loader
                                .items_to_datas(keyboard_ui_sprite_labels.iter(), asset_id);

                            let mut item_entities = position_inits
                                .0
                                .into_iter()
                                .zip(ui_sprite_labels.0.into_iter())
                                .map(|(position_init, ui_sprite_label)| {
                                    let mut item_entity_builder = asset_world
                                        .create_entity()
                                        .with(position_init)
                                        .with(ui_sprite_label)
                                        .with(sequence_end_transitions.clone())
                                        .with(wait_sequence_handles.clone())
                                        .with(tint_sequence_handles.clone())
                                        .with(scale_sequence_handles.clone());

                                    if let Some(sprite_render_sequence_handles) =
                                        sprite_render_sequence_handles.clone()
                                    {
                                        item_entity_builder = item_entity_builder
                                            .with(sprite_render_sequence_handles);
                                    }

                                    item_entity_builder.build()
                                })
                                .collect::<Vec<Entity>>();

                            // Create entity for title label.
                            let item_id_title = {
                                let ui_label = control_settings.title.clone();
                                let position_init = ui_label.position;
                                asset_world
                                    .create_entity()
                                    .with(position_init)
                                    .with(ui_label)
                                    .build()
                            };
                            item_entities.push(item_id_title);
                        }
                    }
                }

                // --- TODO: Delete all code below this. --- //

                // `UiDefinition` specific asset data.
                // `PositionInit`s
                let position_inits = {
                    let mut position_inits = Vec::new();
                    if let Some(background_definition) = background_definition {
                        position_inits.append(&mut PositionInitsLoader::items_to_datas(
                            background_definition.layers.values(),
                        ));
                    }

                    if let Some(ui_definition) = ui_definition {
                        match &ui_definition.ui_type {
                            UiType::Menu(ui_menu_items) => {
                                position_inits.append(&mut PositionInitsLoader::items_to_datas(
                                    ui_menu_items.iter(),
                                ));
                            }
                            UiType::ControlSettings(_control_settings) => {
                                let keyboard_ui_sprite_labels = keyboard_ui_sprite_labels
                                    .as_ref()
                                    .expect("Expected `keyboard_ui_sprite_labels` to exist.");
                                position_inits.append(&mut PositionInitsLoader::items_to_datas(
                                    keyboard_ui_sprite_labels.iter(),
                                ));
                            }
                        }
                    }

                    PositionInits::new(position_inits)
                };
                asset_position_inits.insert(asset_id, position_inits);

                // `UiMenuItem`s
                if let Some(UiDefinition {
                    ui_type: UiType::Menu(ui_menu_items),
                    ..
                }) = ui_definition
                {
                    ui_menu_items_loader.load(ui_menu_items.iter(), asset_id);
                }

                // `UiLabel`s
                if let Some(ui_definition) = ui_definition {
                    let mut ui_labels = Vec::new();
                    match &ui_definition.ui_type {
                        UiType::Menu(ui_menu_items) => {
                            ui_labels
                                .append(&mut UiLabelsLoader::items_to_datas(ui_menu_items.iter()));
                        }
                        UiType::ControlSettings(control_settings) => {
                            ui_labels.append(&mut UiLabelsLoader::items_to_datas(std::iter::once(
                                control_settings,
                            )));
                        }
                    }
                    ui_labels.append(&mut UiLabelsLoader::items_to_datas(
                        ui_definition.buttons.iter(),
                    ));

                    let ui_labels = UiLabels::new(ui_labels);
                    asset_ui_labels.insert(asset_id, ui_labels);
                }

                // `UiSpriteLabel`s
                let ui_sprite_labels = {
                    let mut ui_sprite_labels = Vec::new();
                    if let Some(background_definition) = background_definition {
                        // Background layers.
                        let layers_as_ui_sprite_label =
                            background_definition
                                .layers
                                .iter()
                                .map(|(sequence_string, layer)| {
                                    let sequence_name_string = SequenceNameString::from_str(
                                        sequence_string,
                                    )
                                    .expect("Expected `SequenceNameString::from_str` to succeed.");
                                    config::UiSpriteLabel::new(layer.position, sequence_name_string)
                                });
                        let mut layers_as_ui_sprite_label = ui_sprite_labels_loader
                            .items_to_datas(layers_as_ui_sprite_label, asset_id);
                        ui_sprite_labels.append(&mut layers_as_ui_sprite_label);
                    }

                    if let Some(ui_definition) = ui_definition {
                        match &ui_definition.ui_type {
                            UiType::Menu(ui_menu_items) => {
                                let mut ui_sprite_labels_menu = ui_sprite_labels_loader
                                    .items_to_datas(ui_menu_items.iter(), asset_id);
                                ui_sprite_labels.append(&mut ui_sprite_labels_menu);
                            }
                            UiType::ControlSettings(_control_settings) => {
                                let keyboard_ui_sprite_labels = keyboard_ui_sprite_labels
                                    .as_ref()
                                    .expect("Expected `keyboard_ui_sprite_labels` to exist.");
                                let mut keyboard_ui_sprite_labels = ui_sprite_labels_loader
                                    .items_to_datas(keyboard_ui_sprite_labels.iter(), asset_id);
                                ui_sprite_labels.append(&mut keyboard_ui_sprite_labels);
                            }
                        }

                        let mut ui_sprite_labels_buttons = ui_sprite_labels_loader
                            .items_to_datas(ui_definition.buttons.iter(), asset_id);
                        ui_sprite_labels.append(&mut ui_sprite_labels_buttons);
                    }

                    UiSpriteLabels::new(ui_sprite_labels)
                };
                asset_ui_sprite_labels.insert(asset_id, ui_sprite_labels);

                // Sequence components from both `BackgroundDefinition` and `UiDefinition`.
                let wait_sequence_handles = {
                    let wait_sequence_loader = &wait_sequence_handles_loader.wait_sequence_loader;

                    let mut wait_sequence_handles = Vec::new();

                    if let Some(background_definition) = background_definition {
                        wait_sequence_handles.extend(background_definition.layers.values().map(
                            |layer| {
                                wait_sequence_loader
                                    .load(|frame| frame.wait, layer.sequence.frames.iter())
                            },
                        ));
                    }

                    if let Some(ui_definition) = ui_definition {
                        wait_sequence_handles.extend(ui_definition.sequences.values().map(
                            |sequence| {
                                wait_sequence_loader
                                    .load(|frame| frame.wait, sequence.frames.iter())
                            },
                        ));
                    };

                    WaitSequenceHandles::new(wait_sequence_handles)
                };
                asset_wait_sequence_handles.insert(asset_id, wait_sequence_handles);

                if let Some(sprite_sheet_handles) = sprite_sheet_handles {
                    let sprite_render_sequence_handles = {
                        let sprite_frame_to_sprite_render = |frame: &SpriteFrame| {
                            let sprite_ref = &frame.sprite;
                            let sprite_sheet = sprite_sheet_handles[sprite_ref.sheet].clone();
                            let sprite_number = sprite_ref.index;
                            SpriteRender {
                                sprite_sheet,
                                sprite_number,
                            }
                        };

                        let sprite_render_sequence_loader =
                            &sprite_render_sequence_handles_loader.sprite_render_sequence_loader;

                        let mut sprite_render_sequence_handles = Vec::new();

                        if let Some(background_definition) = background_definition {
                            sprite_render_sequence_handles.extend(
                                background_definition.layers.values().map(|layer| {
                                    sprite_render_sequence_loader.load(
                                        sprite_frame_to_sprite_render,
                                        layer.sequence.frames.iter(),
                                    )
                                }),
                            );
                        }

                        if let Some(ui_definition) = ui_definition {
                            sprite_render_sequence_handles.extend(
                                ui_definition.sequences.values().map(|sequence| {
                                    sprite_render_sequence_loader
                                        .load(sprite_frame_to_sprite_render, sequence.frames.iter())
                                }),
                            );
                        }

                        SpriteRenderSequenceHandles::new(sprite_render_sequence_handles)
                    };

                    asset_sprite_render_sequence_handles
                        .insert(asset_id, sprite_render_sequence_handles);
                }

                let sequence_end_transitions = {
                    let mut sequence_end_transitions = Vec::new();

                    if let Some(background_definition) = background_definition {
                        sequence_end_transitions.extend(background_definition.layers.values().map(
                            |layer| {
                                sequence_end_transition_mapper_ui.map_disparate(asset_id, layer)
                            },
                        ));
                    }

                    if let Some(ui_definition) = ui_definition {
                        sequence_end_transitions.extend(ui_definition.sequences.values().map(
                            |sequence| sequence_end_transition_mapper_ui.map(asset_id, sequence),
                        ));
                    };

                    SequenceEndTransitions::new(sequence_end_transitions)
                };
                asset_sequence_end_transitions.insert(asset_id, sequence_end_transitions);

                let tint_sequence_handles = {
                    let tint_sequence_loader = &tint_sequence_handles_loader.tint_sequence_loader;

                    let mut tint_sequence_handles = Vec::new();

                    if let Some(background_definition) = background_definition {
                        tint_sequence_handles.extend(
                            background_definition.layers.values().map(|layer| {
                                tint_sequence_loader.load(layer.sequence.frames.iter())
                            }),
                        );
                    }

                    if let Some(ui_definition) = ui_definition {
                        tint_sequence_handles.extend(
                            ui_definition
                                .sequences
                                .values()
                                .map(|sequence| tint_sequence_loader.load(sequence.frames.iter())),
                        );
                    };

                    TintSequenceHandles::new(tint_sequence_handles)
                };
                asset_tint_sequence_handles.insert(asset_id, tint_sequence_handles);

                let scale_sequence_handles =
                    {
                        let scale_sequence_loader =
                            &scale_sequence_handles_loader.scale_sequence_loader;

                        let mut scale_sequence_handles = Vec::new();

                        if let Some(background_definition) = background_definition {
                            scale_sequence_handles.extend(
                                background_definition.layers.values().map(|layer| {
                                    scale_sequence_loader.load(layer.sequence.frames.iter())
                                }),
                            );
                        }

                        if let Some(ui_definition) = ui_definition {
                            scale_sequence_handles.extend(ui_definition.sequences.values().map(
                                |sequence| scale_sequence_loader.load(sequence.frames.iter()),
                            ));
                        };

                        ScaleSequenceHandles::new(scale_sequence_handles)
                    };
                asset_scale_sequence_handles.insert(asset_id, scale_sequence_handles);
            }
        }
    }

    /// Returns whether sequence components assets have been loaded.
    fn is_complete(
        _: &AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            wait_sequence_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            tint_sequence_assets,
            scale_sequence_assets,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            asset_tint_sequence_handles,
            asset_scale_sequence_handles,
            ..
        }: &SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        macro_rules! sequence_component_loaded {
            ($handleses:ident, $assets:ident) => {{
                if let Some(handles) = $handleses.get(asset_id) {
                    handles.iter().all(|handle| $assets.get(handle).is_some())
                } else {
                    true
                }
            }};
        };

        sequence_component_loaded!(asset_wait_sequence_handles, wait_sequence_assets)
            && sequence_component_loaded!(asset_source_sequence_handles, source_sequence_assets)
            && sequence_component_loaded!(
                asset_object_acceleration_sequence_handles,
                object_acceleration_sequence_assets
            )
            && sequence_component_loaded!(
                asset_sprite_render_sequence_handles,
                sprite_render_sequence_assets
            )
            && sequence_component_loaded!(asset_body_sequence_handles, body_sequence_assets)
            && sequence_component_loaded!(
                asset_interactions_sequence_handles,
                interactions_sequence_assets
            )
            && sequence_component_loaded!(asset_spawns_sequence_handles, spawns_sequence_assets)
            && sequence_component_loaded!(asset_tint_sequence_handles, tint_sequence_assets)
            && sequence_component_loaded!(asset_scale_sequence_handles, scale_sequence_assets)
    }
}
