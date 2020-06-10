use amethyst::ecs::{Builder, WorldExt};
use application_menu::MenuIndex;
use asset_model::{config::AssetSlug, loaded::ItemId, play::AssetWorld};
use game_input_model::{
    loaded::PlayerControllers,
    play::{ButtonInputControlled, InputControlled, NormalInputControlled, SharedInputControlled},
};
use sequence_loading::SequenceIdMapper;
use sequence_model::loaded::SequenceIdMappings;
use sprite_model::config::SpriteSequenceName;
use ui_menu_item_model::{
    config::UiMenuItems,
    loaded::{UiMenu, UiMenuItem},
};
use ui_model_spi::loaded::WidgetStatusSequences;

use crate::UiAsclComponents;

/// Loads asset items for a `UiMenu`.
#[derive(Debug)]
pub struct UiAsclMenu;

impl UiAsclMenu {
    /// Loads asset items for a `UiMenu`.
    pub fn load(
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        ui_ascl_components: &UiAsclComponents,
        item_ids_all: &mut Vec<ItemId>,
        player_controllers: &PlayerControllers,
        ui_menu_items_cfg: &UiMenuItems<MenuIndex>,
    ) {
        let (ui_menu_item_item_ids, sprite_item_ids) = ui_menu_items_cfg.iter().enumerate().fold(
            (
                Vec::with_capacity(ui_menu_items_cfg.len()),
                Vec::with_capacity(ui_menu_items_cfg.len()),
            ),
            |(mut ui_menu_item_item_ids, mut sprite_item_ids), (index, ui_menu_item_cfg)| {
                let UiAsclComponents {
                    sequence_end_transitions,
                    wait_sequence_handles,
                    tint_sequence_handles,
                    scale_sequence_handles,
                    input_reactions_sequence_handles,
                    sprite_render_sequence_handles,
                } = ui_ascl_components.clone();

                let tab_order = (index as u32) << 1;

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
                    .with(ButtonInputControlled)
                    .with(NormalInputControlled::new(tab_order))
                    .build();
                let item_entity_text = ItemId::new(item_entity_text);

                let ui_sprite_label = &ui_menu_item_cfg.sprite;
                let position_init = ui_menu_item_cfg.position + ui_sprite_label.position;
                let sequence_id_init = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                    sequence_id_mappings,
                    asset_slug,
                    &ui_sprite_label.sequence,
                );
                let widget_status_sequences = ui_menu_item_cfg
                    .widget_status_sequences
                    .iter()
                    .map(|(widget_status, sequence_name_string)| {
                        let sequence_id = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                            sequence_id_mappings,
                            asset_slug,
                            sequence_name_string,
                        );
                        (*widget_status, sequence_id)
                    })
                    .collect::<WidgetStatusSequences>();

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
                        .with(SharedInputControlled)
                        .with(ButtonInputControlled)
                        .with(NormalInputControlled::new(tab_order + 1))
                        .with(widget_status_sequences);

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
            let controller_count = player_controllers.len();
            (0..controller_count)
                .map(InputControlled::new)
                .map(|input_controlled| asset_world.create_entity().with(input_controlled).build())
                .map(ItemId::new)
        };
        item_ids_all.extend(input_controlled_items);
    }
}
