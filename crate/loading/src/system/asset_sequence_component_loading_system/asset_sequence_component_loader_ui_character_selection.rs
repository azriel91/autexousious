use amethyst::ecs::{Builder, WorldExt};
use asset_model::{config::AssetSlug, loaded::ItemId, play::AssetWorld};
use character_selection_ui_model::{
    config::{CharacterSelectionUi, CswLayer, CswLayerName, CswTemplate},
    loaded::{CharacterSelectionWidget, CswPortraits},
    play::CswMain,
};
use game_input::InputControlled;
use game_input_model::InputConfig;
use kinematic_loading::PositionInitsLoader;
use sequence_loading::SequenceIdMapper;
use sequence_model::{config::SequenceNameString, loaded::SequenceIdMappings};
use sprite_model::config::SpriteSequenceName;

use crate::AssetSequenceComponentLoaderUiComponents;

/// Loads asset items for a `CharacterSelection` UI.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUiCharacterSelection;

impl AssetSequenceComponentLoaderUiCharacterSelection {
    /// Loads asset items for a `CharacterSelection` UI.
    pub fn load(
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        item_ids_all: &mut Vec<ItemId>,
        input_config: &InputConfig,
        character_selection_ui: &CharacterSelectionUi,
    ) {
        let CharacterSelectionUi {
            widgets, // Vec<CswDefinition>
            widget_template:
                CswTemplate {
                    portraits:
                        character_selection_ui_model::config::CswPortraits {
                            join,
                            random,
                            select,
                        },
                    layers, // IndexMap<String, UiSpriteLabel>
                },
            characters_available_selector: _,
        } = character_selection_ui;

        // Store widget item IDs in `item_ids_all` to be spawned during state ID
        // updates, but don't store item IDs for widget template layers as those
        // are instantiated when each widget item ID is attached to an entity.

        // Layer item IDs
        let position_inits_widgets = PositionInitsLoader::items_to_datas(widgets.iter());
        let sequence_id_join = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
            sequence_id_mappings,
            asset_slug,
            join,
        );
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
        let csw_portraits = CswPortraits {
            join: sequence_id_join,
            random: sequence_id_random,
            select: sequence_id_select,
        };
        let item_ids_layers = position_inits_widgets
            .0
            .into_iter()
            .map(|position_init_widget| {
                let mut position_inits = PositionInitsLoader::items_to_datas(layers.values());
                position_inits
                    .iter_mut()
                    .for_each(|position_init| *position_init += position_init_widget);
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
                    .map(|((position_init, sequence_id_init), csw_layer)| {
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

                        match csw_layer {
                            CswLayer::Name(CswLayerName::Main) => {
                                item_entity_builder = item_entity_builder.with(CswMain);
                            }
                            CswLayer::Name(CswLayerName::Portrait) => {
                                item_entity_builder = item_entity_builder.with(csw_portraits);
                            }
                            _ => {}
                        }

                        if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles
                        {
                            item_entity_builder =
                                item_entity_builder.with(sprite_render_sequence_handles);
                        }

                        item_entity_builder.build()
                    })
                    .map(ItemId::new)
                    .collect::<Vec<ItemId>>()
            })
            .collect::<Vec<Vec<ItemId>>>();

        // Widget item IDs
        let input_controlleds = {
            let controller_count = input_config.controller_configs.len();
            (0..controller_count)
                .into_iter()
                .map(InputControlled::new)
                .collect::<Vec<InputControlled>>()
        };
        let mut item_ids_widgets = item_ids_layers
            .into_iter()
            .zip(input_controlleds.into_iter())
            .map(
                |(layer_item_ids, input_controlled)| CharacterSelectionWidget {
                    layers: layer_item_ids,
                    input_controlled,
                },
            )
            .map(|character_selection_widget| {
                asset_world
                    .create_entity()
                    .with(character_selection_widget)
                    .build()
            })
            .map(ItemId::new)
            .collect::<Vec<ItemId>>();

        item_ids_all.append(&mut item_ids_widgets);
    }
}
