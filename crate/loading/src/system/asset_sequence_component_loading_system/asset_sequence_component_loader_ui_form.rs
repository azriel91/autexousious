use amethyst::ecs::{Builder, WorldExt};
use asset_model::{config::AssetSlug, loaded::ItemId, play::AssetWorld};
use sequence_loading::SequenceIdMapper;
use sequence_model::loaded::SequenceIdMappings;
use sprite_model::config::SpriteSequenceName;
use ui_form_model::{
    config::{UiFormItem, UiFormItems},
    loaded::UiForm,
};
use ui_model_spi::loaded::WidgetStatusSequences;

use crate::AssetSequenceComponentLoaderUiComponents;

/// Loads asset items for a `UiForm`.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUiForm;

impl AssetSequenceComponentLoaderUiForm {
    /// Loads asset items for a `UiForm`.
    pub fn load(
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        item_ids_all: &mut Vec<ItemId>,
        ui_form_items: &UiFormItems,
    ) {
        let ui_form_item_item_ids = ui_form_items.iter().fold(
            Vec::with_capacity(ui_form_items.len()),
            |mut ui_form_item_item_ids, ui_form_item| {
                let item_entity_label = Self::load_item_entity_label(asset_world, ui_form_item);
                let item_entity_sprite = Self::load_item_entity_sprite(
                    asset_world,
                    asset_slug,
                    sequence_id_mappings,
                    asset_sequence_component_loader_ui_components,
                    ui_form_item,
                );
                let item_entity_input = Self::load_item_entity_input(asset_world, ui_form_item);

                ui_form_item_item_ids.push((
                    item_entity_label,
                    item_entity_sprite,
                    item_entity_input,
                ));
                ui_form_item_item_ids
            },
        );

        let ui_form_item_id = {
            let ui_menu = UiForm::new(ui_form_item_item_ids);
            let item_entity = asset_world.create_entity().with(ui_menu).build();

            ItemId::new(item_entity)
        };

        item_ids_all.push(ui_form_item_id);
    }

    fn load_item_entity_label(asset_world: &mut AssetWorld, ui_form_item: &UiFormItem) -> ItemId {
        let mut ui_label = ui_form_item.label.clone();
        ui_label.position += ui_form_item.position;
        let position_init = ui_label.position;
        let item_entity_label = asset_world
            .create_entity()
            .with(position_init)
            .with(ui_label)
            .build();
        ItemId::new(item_entity_label)
    }

    fn load_item_entity_sprite(
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        ui_form_item: &UiFormItem,
    ) -> ItemId {
        let ui_sprite_label = &ui_form_item.sprite;
        let position_init = ui_form_item.position + ui_sprite_label.position;
        let sequence_id_init = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
            sequence_id_mappings,
            asset_slug,
            &ui_sprite_label.sequence,
        );
        let widget_status_sequences = ui_form_item
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
                .with(input_reactions_sequence_handles)
                .with(widget_status_sequences);

            if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                item_entity_builder = item_entity_builder.with(sprite_render_sequence_handles);
            }

            item_entity_builder.build()
        };
        ItemId::new(item_entity_sprite)
    }

    fn load_item_entity_input(asset_world: &mut AssetWorld, ui_form_item: &UiFormItem) -> ItemId {
        let mut ui_text_input = ui_form_item.input_field.clone();
        ui_text_input.label_attributes.position += ui_form_item.position;
        let position_init = ui_text_input.label_attributes.position;
        let item_entity_text = asset_world
            .create_entity()
            .with(position_init)
            .with(ui_text_input)
            .build();
        ItemId::new(item_entity_text)
    }
}
