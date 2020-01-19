use amethyst::ecs::{Builder, Entity, WorldExt};
use asset_model::{config::AssetSlug, loaded::ItemId, play::AssetWorld};
use game_mode_selection_model::GameModeIndex;
use sequence_loading::SequenceIdMapper;
use sequence_model::loaded::SequenceIdMappings;
use smallvec::SmallVec;
use sprite_model::config::SpriteSequenceName;
use ui_menu_item_model::{config::UiMenuItems, loaded::UiMenuItem};

use crate::AssetSequenceComponentLoaderUiComponents;

/// Loads asset items for a `UiMenu`.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUiMenu;

impl AssetSequenceComponentLoaderUiMenu {
    /// Loads asset items for a `UiMenu`.
    pub fn load(
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        item_ids_all: &mut Vec<ItemId>,
        ui_menu_items_cfg: &UiMenuItems<GameModeIndex>,
    ) {
        let mut item_ids = ui_menu_items_cfg
            .iter()
            .flat_map(|ui_menu_item_cfg| {
                let mut ui_label = ui_menu_item_cfg.label.clone();
                ui_label.position += ui_menu_item_cfg.position;
                let ui_menu_item = UiMenuItem::new(ui_menu_item_cfg.index);
                let position_init = ui_label.position;
                let item_entity_text = asset_world
                    .create_entity()
                    .with(position_init)
                    .with(ui_label)
                    .with(ui_menu_item)
                    .build();

                let ui_sprite_label = &ui_menu_item_cfg.sprite;
                let position_init = ui_menu_item_cfg.position + ui_sprite_label.position;
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

                let item_entity_sprite = {
                    let mut item_entity_builder = asset_world
                        .create_entity()
                        .with(position_init)
                        .with(sequence_id_init)
                        .with(sequence_end_transitions)
                        .with(wait_sequence_handles)
                        .with(tint_sequence_handles)
                        .with(scale_sequence_handles)
                        .with(input_reactions_sequence_handles);

                    if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                        item_entity_builder =
                            item_entity_builder.with(sprite_render_sequence_handles);
                    }

                    item_entity_builder.build()
                };

                let item_entities =
                    SmallVec::<[Entity; 2]>::from_buf([item_entity_text, item_entity_sprite]);
                item_entities.into_iter()
            })
            .map(ItemId::new)
            .collect::<Vec<ItemId>>();

        item_ids_all.append(&mut item_ids);
    }
}
