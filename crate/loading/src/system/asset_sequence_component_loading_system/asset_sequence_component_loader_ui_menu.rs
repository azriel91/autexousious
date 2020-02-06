use amethyst::ecs::{Builder, WorldExt};
use application_menu::MenuIndex;
use asset_model::{config::AssetSlug, loaded::ItemId, play::AssetWorld};
use game_input_model::{
    config::InputConfig,
    play::{InputControlled, SharedInputControlled},
};
use sequence_loading::SequenceIdMapper;
use sequence_model::loaded::SequenceIdMappings;
use sprite_model::config::SpriteSequenceName;
use ui_menu_item_model::{
    config::UiMenuItems,
    loaded::{UiMenu, UiMenuItem},
};

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
        input_config: &InputConfig,
        ui_menu_items_cfg: &UiMenuItems<MenuIndex>,
    ) {
        let (ui_menu_item_item_ids, sprite_item_ids) = ui_menu_items_cfg.iter().fold(
            (
                Vec::with_capacity(ui_menu_items_cfg.len()),
                Vec::with_capacity(ui_menu_items_cfg.len()),
            ),
            |(mut ui_menu_item_item_ids, mut sprite_item_ids), ui_menu_item_cfg| {
                let AssetSequenceComponentLoaderUiComponents {
                    sequence_end_transitions,
                    wait_sequence_handles,
                    tint_sequence_handles,
                    scale_sequence_handles,
                    input_reactions_sequence_handles,
                    sprite_render_sequence_handles,
                } = asset_sequence_component_loader_ui_components.clone();

                let mut ui_label = ui_menu_item_cfg.label.clone();
                ui_label.position += ui_menu_item_cfg.position;
                let ui_menu_item = UiMenuItem::new(ui_menu_item_cfg.index);
                let position_init = ui_label.position;
                let item_entity_text = asset_world
                    .create_entity()
                    .with(position_init)
                    .with(ui_label)
                    .with(ui_menu_item)
                    .with(SharedInputControlled)
                    .build();
                let item_entity_text = ItemId::new(item_entity_text);

                let ui_sprite_label = &ui_menu_item_cfg.sprite;
                let position_init = ui_menu_item_cfg.position + ui_sprite_label.position;
                let sequence_id_init = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                    sequence_id_mappings,
                    asset_slug,
                    &ui_sprite_label.sequence,
                );

                let item_entity_sprite = {
                    let mut item_entity_builder = asset_world
                        .create_entity()
                        .with(position_init)
                        .with(sequence_id_init)
                        .with(sequence_end_transitions)
                        .with(wait_sequence_handles)
                        .with(tint_sequence_handles)
                        .with(scale_sequence_handles)
                        .with(input_reactions_sequence_handles)
                        .with(SharedInputControlled);

                    if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                        item_entity_builder =
                            item_entity_builder.with(sprite_render_sequence_handles);
                    }

                    item_entity_builder.build()
                };
                let item_entity_sprite = ItemId::new(item_entity_sprite);

                ui_menu_item_item_ids.push(item_entity_text);
                sprite_item_ids.push(item_entity_sprite);

                (ui_menu_item_item_ids, sprite_item_ids)
            },
        );

        let ui_menu_item_id = {
            let ui_menu = UiMenu::new(ui_menu_item_item_ids, sprite_item_ids);
            let item_entity = asset_world.create_entity().with(ui_menu).build();

            ItemId::new(item_entity)
        };

        item_ids_all.push(ui_menu_item_id);

        // Since the `UiLabel` entities use a `SharedInputControlled`, we still need entities with
        // `InputControlled` to that the individual `ControllerInput`s are stored against.
        let input_controlled_items = {
            let controller_count = input_config.controller_configs.len();
            (0..controller_count)
                .map(InputControlled::new)
                .map(|input_controlled| asset_world.create_entity().with(input_controlled).build())
                .map(ItemId::new)
        };
        item_ids_all.extend(input_controlled_items);
    }
}
