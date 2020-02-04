use amethyst::ecs::{Builder, WorldExt};
use asset_model::{config::AssetSlug, loaded::ItemId, play::AssetWorld};
use control_settings_model::config::{ControlButtonLabels, ControlSettings};
use game_input_model::play::ButtonInputControlled;
use kinematic_loading::PositionInitsLoader;
use sequence_loading::SequenceIdMapper;
use sequence_model::loaded::SequenceIdMappings;
use sprite_model::config::SpriteSequenceName;

use crate::AssetSequenceComponentLoaderUiComponents;

/// Loads asset items for a `ControlSettings` UI.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUiControlSettings;

impl AssetSequenceComponentLoaderUiControlSettings {
    /// Loads asset items for a `ControlSettings` UI.
    pub fn load(
        asset_world: &mut AssetWorld,
        asset_slug: &AssetSlug,
        sequence_id_mappings: &SequenceIdMappings<SpriteSequenceName>,
        asset_sequence_component_loader_ui_components: &AssetSequenceComponentLoaderUiComponents,
        item_ids_all: &mut Vec<ItemId>,
        control_settings: &ControlSettings,
        keyboard_button_labels: &ControlButtonLabels,
    ) {
        let position_inits = PositionInitsLoader::items_to_datas(keyboard_button_labels.iter());
        let sequence_id_inits = SequenceIdMapper::<SpriteSequenceName>::items_to_datas(
            sequence_id_mappings,
            asset_slug,
            keyboard_button_labels
                .iter()
                .map(|control_button_label| &control_button_label.sprite.sequence),
        );

        let mut item_ids = position_inits
            .0
            .into_iter()
            .zip(sequence_id_inits.into_iter())
            .map(|(position_init, sequence_id_init)| {
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
                    .with(input_reactions_sequence_handles)
                    .with(ButtonInputControlled);

                if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                    item_entity_builder = item_entity_builder.with(sprite_render_sequence_handles);
                }

                item_entity_builder.build()
            })
            .map(ItemId::new)
            .collect::<Vec<ItemId>>();

        // Create entity for title label.
        let item_id_title = {
            let ui_label = control_settings.title.clone();
            let position_init = ui_label.position;
            let entity = asset_world
                .create_entity()
                .with(position_init)
                .with(ui_label)
                .build();
            ItemId::new(entity)
        };
        item_ids.push(item_id_title);

        item_ids_all.append(&mut item_ids);
    }
}
