use amethyst::ecs::{Builder, WorldExt};
use asset_model::loaded::{AssetId, ItemId, ItemIds};
use character_loading::CHARACTER_INPUT_REACTIONS_DEFAULT;
use character_model::{
    config::{CharacterSequence, CharacterSequenceName},
    loaded::{CharacterIrsHandle, CharacterIrsHandles},
};
use energy_model::config::{EnergySequence, EnergySequenceName};
use input_reaction_loading::{IrsLoader, IrsLoaderParams};
use kinematic_model::{
    config::{PositionInit, VelocityInit},
    play::PositionZAsY,
};
use loading_spi::{
    AssetLoadingResources, DefinitionLoadingResourcesRead, IdMappingResourcesRead,
    SequenceComponentLoadingResources, TextureLoadingResourcesRead,
};
use log::warn;
use mirrored_model::play::Mirrored;
use object_loading::{ObjectLoader, ObjectLoaderParams};
use object_model::{loaded::Object, play::Grounding};
use object_type::ObjectType;
use sequence_model::loaded::SequenceId;

/// Loads sequence components for object assets.
#[derive(Debug)]
pub struct ObjectAscl;

impl ObjectAscl {
    /// Loads sequence components for object assets.
    pub fn load(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    ..
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
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
            asset_item_ids,
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
            character_input_reactions_assets,
            character_irs_assets,
            ..
        }: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
        object_type: ObjectType,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            loader,
            ..
        } = asset_loading_resources;

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        let mut item_entity_builder = asset_world.create_entity();

        let sprite_sheet_handles = asset_sprite_sheet_handles
            .get(asset_id)
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

        let (sequence_id_init, object) = match object_type {
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
                let sequence_id_init = {
                    let sequence_name_default = CharacterSequenceName::default();
                    sequence_id_mappings
                        .id_by_name(sequence_name_default)
                        .copied()
                        .unwrap_or_else(|| {
                            warn!(
                                "`{}` sequence ID not found for asset: `{}`. \
                                 Falling back to first declared sequence.",
                                sequence_name_default, asset_slug
                            );

                            SequenceId::new(0)
                        })
                };

                let irs_loader_params = IrsLoaderParams {
                    loader,
                    input_reactions_assets: &*character_input_reactions_assets,
                    input_reactions_sequence_assets: &*character_irs_assets,
                };

                let character_irs_handles = {
                    let character_irs_handles = character_definition
                        .object_definition
                        .sequences
                        .iter()
                        .map(|(sequence_id, sequence)| {
                            let sequence_default = CHARACTER_INPUT_REACTIONS_DEFAULT
                                .object_definition
                                .sequences
                                .get(sequence_id);

                            IrsLoader::load(
                                &irs_loader_params,
                                sequence_id_mappings,
                                sequence_default,
                                sequence,
                            )
                        })
                        .collect::<Vec<CharacterIrsHandle>>();
                    CharacterIrsHandles::new(character_irs_handles)
                };

                item_entity_builder = item_entity_builder.with(character_irs_handles);

                let object = ObjectLoader::load::<CharacterSequence>(
                    object_loader_params,
                    &character_definition.object_definition,
                );

                (sequence_id_init, object)
            }
            ObjectType::Energy => {
                let energy_definition = asset_energy_definition_handle
                    .get(asset_id)
                    .and_then(|energy_definition_handle| {
                        energy_definition_assets.get(energy_definition_handle)
                    })
                    .expect("Expected `EnergyDefinition` to be loaded.");

                let sequence_id_mappings = asset_sequence_id_mappings_energy
                    .get(asset_id)
                    .expect("Expected `SequenceIdMapping` to be loaded.");
                let sequence_id_init = {
                    let sequence_name_default = EnergySequenceName::default();
                    sequence_id_mappings
                        .id_by_name(sequence_name_default)
                        .copied()
                        .unwrap_or_else(|| {
                            warn!(
                                "`{}` sequence ID not found for asset: `{}`. \
                                 Falling back to first declared sequence.",
                                sequence_name_default, asset_slug
                            );

                            SequenceId::new(0)
                        })
                };

                let object = ObjectLoader::load::<EnergySequence>(
                    object_loader_params,
                    &energy_definition.object_definition,
                );

                (sequence_id_init, object)
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

        let item_id = {
            let item_entity = item_entity_builder
                .with(PositionInit::new(0, 0, 0))
                .with(VelocityInit::new(0, 0, 0))
                .with(PositionZAsY)
                .with(Mirrored::default())
                .with(Grounding::default())
                .with(sequence_id_init)
                .with(sequence_end_transitions)
                .with(wait_sequence_handles)
                .with(source_sequence_handles)
                .with(object_acceleration_sequence_handles)
                .with(sprite_render_sequence_handles)
                .with(body_sequence_handles)
                .with(interactions_sequence_handles)
                .with(spawns_sequence_handles)
                .build();
            ItemId::new(item_entity)
        };

        let item_ids = ItemIds::new(vec![item_id]);
        asset_item_ids.insert(asset_id, item_ids);
    }
}
