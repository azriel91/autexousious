use std::{convert::TryFrom, iter};

use amethyst::ecs::{Builder, WorldExt};
use asset_model::{
    config::{AssetSlug, AssetType},
    loaded::{AssetTypeMappings, ItemId},
    play::AssetWorld,
};
use asset_selection_ui_model::{loaded::AssetPreviewWidget, play::ApwMain};
use asset_ui_model::{
    config::{self, AssetDisplay, AssetDisplayGrid, AssetDisplayLayout},
    loaded::{
        AssetDisplayCellMap, AssetSelectionCell, AssetSelectionHighlight, AssetSelector,
        AswPortraits,
    },
    play::{AssetSelectionHighlightMain, AssetSelectionStatus},
};
use chase_model::play::ChaseModeStick;
use game_input_model::{
    config::InputConfig,
    play::{InputControlled, SharedInputControlled},
};
use kinematic_loading::PositionInitsLoader;
use kinematic_model::config::{Position, PositionInit};
use map_selection_ui_model::config::{MapSelectionUi, MpwTemplate, MswLayer, MswLayerName};
use sequence_loading::SequenceIdMapper;
use sequence_model::{config::SequenceNameString, loaded::SequenceIdMappings};
use sprite_model::config::SpriteSequenceName;

use crate::AssetSequenceComponentLoaderUiComponents;

/// Loads asset items for a `MapSelection` UI.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUiMapSelection;

impl AssetSequenceComponentLoaderUiMapSelection {
    /// Loads asset items for a `MapSelection` UI.
    pub fn load(
        asset_type_mappings: &AssetTypeMappings,
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        item_ids_all: &mut Vec<ItemId>,
        input_config: &InputConfig,
        map_selection_ui: &MapSelectionUi,
    ) {
        let MapSelectionUi {
            map_preview:
                MpwTemplate {
                    position: position_map_preview,
                    dimensions: dimensions_map_preview,
                    portraits,
                    layers, // IndexMap<String, UiSpriteLabel>
                },
            maps_available_selector,
        } = map_selection_ui;
        let position_map_preview = *position_map_preview;
        let dimensions_map_preview = *dimensions_map_preview;

        // Store widget item IDs in `item_ids_all` to be spawned during state ID
        // updates, but don't store item IDs for widget template layers as those
        // are instantiated when each widget item ID is attached to an entity.

        // Layer item IDs
        let asw_portraits = portraits
            .iter()
            .map(|(asw_portrait_name, sequence_name_string)| {
                let sequence_id = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                    sequence_id_mappings,
                    asset_slug,
                    sequence_name_string,
                );
                (*asw_portrait_name, sequence_id)
            })
            .collect::<AswPortraits>();
        let item_id_map_preview_layers = {
            let mut position_inits = PositionInitsLoader::items_to_datas(layers.values());
            position_inits
                .iter_mut()
                .for_each(|position_init| *position_init += position_map_preview);
            let sequence_id_inits = SequenceIdMapper::<SpriteSequenceName>::items_to_datas(
                sequence_id_mappings,
                asset_slug,
                layers.values().map(AsRef::<SequenceNameString<_>>::as_ref),
            );

            position_inits
                .0
                .into_iter()
                .zip(sequence_id_inits.into_iter())
                .zip(layers.keys())
                .map(|((position_init, sequence_id_init), msw_layer)| {
                    let AssetSequenceComponentLoaderUiComponents {
                        sequence_end_transitions,
                        wait_sequence_handles,
                        tint_sequence_handles,
                        scale_sequence_handles,
                        input_reactions_sequence_handles,
                        sprite_render_sequence_handles,
                    } = asset_sequence_component_loader_ui_components.clone();

                    let mut item_entity_builder = asset_world
                        .create_entity()
                        .with(position_init)
                        .with(sequence_id_init)
                        .with(sequence_end_transitions)
                        .with(wait_sequence_handles)
                        .with(tint_sequence_handles)
                        .with(scale_sequence_handles)
                        .with(input_reactions_sequence_handles);

                    match msw_layer {
                        MswLayer::Name(MswLayerName::Main) => {
                            item_entity_builder = item_entity_builder
                                .with(ApwMain)
                                .with(dimensions_map_preview);
                        }
                        MswLayer::Name(MswLayerName::Portrait) => {
                            item_entity_builder = item_entity_builder.with(asw_portraits.clone());
                        }
                        _ => {}
                    }

                    if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                        item_entity_builder =
                            item_entity_builder.with(sprite_render_sequence_handles);
                    }

                    item_entity_builder.build()
                })
                .map(ItemId::new)
                .collect::<Vec<ItemId>>()
        };

        // Widget item ID
        let item_id_map_preview = {
            let map_preview_widget = AssetPreviewWidget {
                layers: item_id_map_preview_layers,
                input_controlled: None,
                shared_input_controlled: Some(SharedInputControlled),
            };
            let item_entity = asset_world.create_entity().with(map_preview_widget).build();

            ItemId::new(item_entity)
        };

        item_ids_all.push(item_id_map_preview);
        item_ids_all.push(Self::asset_selector_item(
            asset_type_mappings,
            asset_world,
            asset_slug,
            sequence_id_mappings,
            asset_sequence_component_loader_ui_components,
            maps_available_selector,
        ));

        // Since the `AssetSelectionHighlight` entities use a `SharedInputControlled`, we still need
        // entities with `InputControlled` to that the individual `ControllerInput`s are stored
        // against.
        let input_controlled_items = {
            let controller_count = input_config.controller_configs.len();
            (0..controller_count)
                .map(InputControlled::new)
                .map(|input_controlled| asset_world.create_entity().with(input_controlled).build())
                .map(ItemId::new)
        };
        item_ids_all.extend(input_controlled_items);
    }

    fn asset_selector_item<T>(
        asset_type_mappings: &AssetTypeMappings,
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        maps_available_selector: &config::AssetSelector<T>,
    ) -> ItemId
    where
        T: Default + Into<AssetType> + Send + Sync + 'static,
    {
        let config::AssetSelector {
            asset_display: AssetDisplay {
                position, layout, ..
            },
            selection_highlights,
        } = maps_available_selector;

        let AssetDisplayLayout::Grid(AssetDisplayGrid {
            column_count,
            cell_size,
        }) = *layout;

        // `AssetId`s for the asset type to display.
        //
        // We want to create an item for each of these in the correct place in the grid.
        let asset_type = Into::<AssetType>::into(T::default());
        let asset_display_cell_item_ids = iter::once(AssetSelectionCell::Random)
            .chain(
                asset_type_mappings
                    .iter_ids(&asset_type)
                    .copied()
                    .map(|asset_id| AssetSelectionCell::Id {
                        display_cell: AssetDisplayCellMap {
                            asset_id,
                            cell_size,
                        },
                    }),
            )
            .enumerate()
            .map(|(grid_index, asset_selection_cell)| {
                let column_index = grid_index % column_count;
                let column_index = u32::try_from(column_index).unwrap_or_else(|e| {
                    panic!("Failed to convert `column_index` to `u32`. Error: {}", e)
                });
                let row_index = grid_index / column_count;
                let row_index = u32::try_from(row_index).unwrap_or_else(|e| {
                    panic!("Failed to convert `row_index` to `u32`. Error: {}", e)
                });

                let x = column_index * cell_size.w;
                let x = i32::try_from(x).unwrap_or_else(|e| {
                    panic!(
                        "Failed to convert asset cell position `x` to `i32`. Error: {}",
                        e
                    )
                });
                let y = row_index * cell_size.h;
                let y = i32::try_from(y).unwrap_or_else(|e| {
                    panic!(
                        "Failed to convert asset cell position `y` to `i32`. Error: {}",
                        e
                    )
                });
                let position_asset_cell =
                    PositionInit::new(position.x + x, position.y + y, position.z);

                let item_entity = asset_world
                    .create_entity()
                    .with(position_asset_cell)
                    .with(asset_selection_cell)
                    .build();
                ItemId::new(item_entity)
            })
            .collect::<Vec<ItemId>>();

        // Create item for each `AssetSelectionHighlight`.
        let asset_selection_highlight_item_ids = selection_highlights
            .iter()
            .map(|ash_template| {
                let ui_sprite_label = &ash_template.sprite;
                let position_init = ui_sprite_label.position;
                let offset = Position::<f32>::from(position_init);

                let sequence_id_init = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                    sequence_id_mappings,
                    asset_slug,
                    &ui_sprite_label.sequence,
                );

                let AssetSequenceComponentLoaderUiComponents {
                    sequence_end_transitions,
                    wait_sequence_handles,
                    tint_sequence_handles,
                    scale_sequence_handles,
                    input_reactions_sequence_handles,
                    sprite_render_sequence_handles,
                } = asset_sequence_component_loader_ui_components.clone();

                let chase_mode_stick = ChaseModeStick::new(Some(offset));
                let mut item_entity_builder = asset_world
                    .create_entity()
                    .with(position_init)
                    .with(sequence_id_init)
                    .with(chase_mode_stick)
                    .with(SharedInputControlled)
                    .with(sequence_end_transitions)
                    .with(wait_sequence_handles)
                    .with(tint_sequence_handles)
                    .with(scale_sequence_handles)
                    .with(input_reactions_sequence_handles);

                if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                    item_entity_builder = item_entity_builder.with(sprite_render_sequence_handles);
                }

                let item_entity = item_entity_builder.build();

                ItemId::new(item_entity)
            })
            .collect::<Vec<ItemId>>() // Collect to reclaim `asset_world` for next closure.
            .into_iter()
            .map(|ash_sprite_item_id| {
                let asset_selection_highlight = AssetSelectionHighlight { ash_sprite_item_id };
                let item_entity = asset_world
                    .create_entity()
                    // `StickToTargetObjectSystem` doesn't insert `Position` / `Transform` if it
                    // isn't already there.
                    .with(PositionInit::default())
                    .with(ChaseModeStick::default())
                    .with(asset_selection_highlight)
                    .with(AssetSelectionHighlightMain)
                    .with(AssetSelectionStatus::InProgress)
                    .build();

                ItemId::new(item_entity)
            })
            .collect::<Vec<ItemId>>();

        let asset_selector = AssetSelector::<T>::new(
            asset_display_cell_item_ids,
            asset_selection_highlight_item_ids,
            *layout,
        );
        let item_entity = asset_world
            .create_entity()
            .with(*position)
            .with(asset_selector)
            .build();

        ItemId::new(item_entity)
    }
}
