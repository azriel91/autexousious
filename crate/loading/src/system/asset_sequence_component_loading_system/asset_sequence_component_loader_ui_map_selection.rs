use std::{convert::TryFrom, iter};

use amethyst::ecs::{Builder, WorldExt};
use asset_model::{
    config::{AssetSlug, AssetType},
    loaded::{AssetTypeMappings, ItemId},
    play::AssetWorld,
};
use asset_ui_model::{
    config::{self, AssetDisplay, AssetDisplayGrid, AssetDisplayLayout},
    loaded::{AssetDisplayCellMap, AssetSelectionCell, AssetSelectionHighlight, AssetSelector},
    play::{AssetSelectionHighlightMain, AssetSelectionStatus},
};
use chase_model::play::ChaseModeStick;
use game_input::{InputControlled, SharedInputControlled};
use game_input_model::InputConfig;
use kinematic_loading::PositionInitsLoader;
use kinematic_model::config::{Position, PositionInit};
use map_selection_ui_model::{
    config::{MapSelectionUi, MswLayer, MswLayerName, MswTemplate},
    loaded::{MapSelectionWidget, MswPortraits},
    play::MswMain,
};
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
                MswTemplate {
                    position: position_map_preview,
                    portraits: map_selection_ui_model::config::MswPortraits { random, select },
                    layers, // IndexMap<String, UiSpriteLabel>
                },
            maps_available_selector,
        } = map_selection_ui;

        let input_controlleds = {
            let controller_count = input_config.controller_configs.len();
            (0..controller_count)
                .into_iter()
                .map(InputControlled::new)
                .collect::<Vec<InputControlled>>()
        };

        // Store widget item IDs in `item_ids_all` to be spawned during state ID
        // updates, but don't store item IDs for widget template layers as those
        // are instantiated when each widget item ID is attached to an entity.

        // Layer item IDs
        let sequence_id_random = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
            sequence_id_mappings,
            asset_slug,
            random,
        );
        let sequence_id_select = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
            sequence_id_mappings,
            asset_slug,
            select,
        );
        let msw_portraits = MswPortraits {
            random: sequence_id_random,
            select: sequence_id_select,
        };
        let item_id_map_preview_layers = {
            let mut position_inits = PositionInitsLoader::items_to_datas(layers.values());
            position_inits
                .iter_mut()
                .for_each(|position_init| *position_init += *position_map_preview);
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
                            item_entity_builder = item_entity_builder.with(MswMain);
                        }
                        MswLayer::Name(MswLayerName::Portrait) => {
                            item_entity_builder = item_entity_builder.with(msw_portraits);
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
            let map_selection_widget = MapSelectionWidget {
                layers: item_id_map_preview_layers,
            };
            let item_entity = asset_world
                .create_entity()
                .with(map_selection_widget)
                .with(SharedInputControlled)
                .build();

            ItemId::new(item_entity)
        };

        item_ids_all.push(item_id_map_preview);
        item_ids_all.push(Self::asset_selector_item(
            asset_type_mappings,
            asset_world,
            asset_slug,
            sequence_id_mappings,
            asset_sequence_component_loader_ui_components,
            &input_controlleds,
            maps_available_selector,
        ));
    }

    fn asset_selector_item<T>(
        asset_type_mappings: &AssetTypeMappings,
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        input_controlleds: &[InputControlled],
        maps_available_selector: &config::AssetSelector<T>,
    ) -> ItemId
    where
        T: Default + Into<AssetType> + Send + Sync + 'static,
    {
        let config::AssetSelector {
            asset_display:
                AssetDisplay {
                    position,
                    layout,
                    marker: _,
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
            .zip(input_controlleds.iter().copied())
            .map(|(ash_template, input_controlled)| {
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
                    .with(input_controlled)
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
            .zip(input_controlleds.iter().copied())
            .map(|(ash_sprite_item_id, input_controlled)| {
                let asset_selection_highlight = AssetSelectionHighlight {
                    ash_sprite_item_id,
                    asset_selection_status: AssetSelectionStatus::Inactive,
                };
                let item_entity = asset_world
                    .create_entity()
                    // `StickToTargetObjectSystem` doesn't insert `Position` / `Transform` if it
                    // isn't already there.
                    .with(PositionInit::default())
                    .with(input_controlled)
                    .with(ChaseModeStick::default())
                    .with(asset_selection_highlight)
                    .with(AssetSelectionHighlightMain)
                    .with(AssetSelectionStatus::Inactive)
                    .build();

                ItemId::new(item_entity)
            })
            .collect::<Vec<ItemId>>();

        let asset_selector_item = {
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
        };

        asset_selector_item
    }
}
