use amethyst::ecs::{Builder, WorldExt};
use asset_model::loaded::{AssetId, ItemId, ItemIds};
use kinematic_loading::PositionInitsLoader;
use map_model::loaded::Margins;
use sequence_loading::{
    SequenceEndTransitionsLoader, SequenceIdMapper, WaitSequenceHandlesLoader, WaitSequenceLoader,
};
use sprite_loading::{
    ScaleSequenceHandlesLoader, ScaleSequenceLoader, SpriteRenderSequenceHandlesLoader,
    SpriteRenderSequenceLoader, TintSequenceHandlesLoader, TintSequenceLoader,
};
use sprite_model::config::SpriteSequenceName;

use crate::{
    AssetLoadingResources, DefinitionLoadingResourcesRead, IdMappingResourcesRead,
    SequenceComponentLoadingResources, TextureLoadingResourcesRead,
};

/// Loads sequence components for map assets.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderMap;

impl AssetSequenceComponentLoaderMap {
    /// Loads sequence components for map assets.
    pub fn load(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    map_definition_assets,
                    asset_map_definition_handle,
                    ..
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    asset_sequence_id_mappings_sprite,
                    ..
                },
            texture_loading_resources_read:
                TextureLoadingResourcesRead {
                    asset_sprite_sheet_handles,
                    ..
                },
            asset_world,
            asset_item_ids,
            wait_sequence_assets,
            sprite_render_sequence_assets,
            tint_sequence_assets,
            scale_sequence_assets,
            asset_map_bounds,
            asset_margins,
            ..
        }: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            loader,
            ..
        } = asset_loading_resources;

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);

        let wait_sequence_loader = WaitSequenceLoader {
            loader,
            wait_sequence_assets,
        };
        let mut wait_sequence_handles_loader = WaitSequenceHandlesLoader {
            wait_sequence_loader,
        };
        let tint_sequence_loader = TintSequenceLoader {
            loader,
            tint_sequence_assets,
        };
        let tint_sequence_handles_loader = TintSequenceHandlesLoader {
            tint_sequence_loader,
        };
        let scale_sequence_loader = ScaleSequenceLoader {
            loader,
            scale_sequence_assets,
        };
        let scale_sequence_handles_loader = ScaleSequenceHandlesLoader {
            scale_sequence_loader,
        };
        let sprite_render_sequence_loader = SpriteRenderSequenceLoader {
            loader,
            sprite_render_sequence_assets,
        };
        let sprite_render_sequence_handles_loader = SpriteRenderSequenceHandlesLoader {
            sprite_render_sequence_loader,
        };

        // Begin

        let map_definition = asset_map_definition_handle
            .get(asset_id)
            .and_then(|map_definition_handle| map_definition_assets.get(map_definition_handle))
            .expect("Expected `MapDefinition` to be loaded.");

        let sequence_id_mappings = asset_sequence_id_mappings_sprite
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for asset `{}`.",
                    asset_slug
                )
            });

        let sequence_end_transitions_loader = SequenceEndTransitionsLoader {
            sequence_id_mappings,
        };

        let background_definition = &map_definition.background;

        let position_inits =
            PositionInitsLoader::items_to_datas(background_definition.layers.values());
        let sequence_id_mappings = asset_sequence_id_mappings_sprite
            .get(asset_id)
            .expect("Expected `SequenceIdMappings` to be loaded.");
        let sequence_id_inits = SequenceIdMapper::<SpriteSequenceName>::strings_to_ids(
            sequence_id_mappings,
            asset_slug,
            background_definition.layers.keys(),
        );
        let sequence_end_transitions = sequence_end_transitions_loader
            .items_to_datas(background_definition.layers.values(), asset_slug);
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
        let sprite_render_sequence_handles = sprite_sheet_handles.map(|sprite_sheet_handles| {
            sprite_render_sequence_handles_loader.items_to_datas(
                background_definition.layers.values(),
                |layer| layer.sequence.frames.iter(),
                sprite_sheet_handles,
            )
        });

        let item_ids = position_inits
            .0
            .into_iter()
            .zip(sequence_id_inits.into_iter())
            .map(|(position_init, sequence_id_init)| {
                let mut item_entity_builder = asset_world
                    .create_entity()
                    .with(position_init)
                    .with(sequence_id_init)
                    .with(sequence_end_transitions.clone())
                    .with(wait_sequence_handles.clone())
                    .with(tint_sequence_handles.clone())
                    .with(scale_sequence_handles.clone());

                if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles.clone()
                {
                    item_entity_builder = item_entity_builder.with(sprite_render_sequence_handles);
                }

                item_entity_builder.build()
            })
            .map(ItemId::new)
            .collect::<Vec<ItemId>>();

        let item_ids = ItemIds::new(item_ids);
        asset_item_ids.insert(asset_id, item_ids);

        let map_bounds = map_definition.header.bounds;
        asset_map_bounds.insert(asset_id, map_bounds);

        let margins = Margins::from(map_bounds);
        asset_margins.insert(asset_id, margins);
    }
}
