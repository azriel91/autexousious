use amethyst::{
    assets::AssetStorage,
    core::{nalgebra::Vector3, transform::Transform},
    ecs::{prelude::*, world::EntitiesRes},
    renderer::{SpriteRender, Transparent},
};
use map_model::loaded::{Map, MapHandle};
use sequence_model::loaded::ComponentSequencesHandle;

use crate::{MapLayerComponentStorages, MapSpawningResources};

/// Spawns map layer entities into the world.
#[derive(Debug)]
pub struct MapLayerEntitySpawner;

impl MapLayerEntitySpawner {
    /// Spawns entities for each of the layers in a map.
    ///
    /// Idea: What if we could spawn two maps at the same time?
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to spawn the map into.
    /// * `map_handle`: Handle of the map whose layers to spawn.
    pub fn spawn_world(world: &mut World, map_handle: &MapHandle) -> Vec<Entity> {
        let entities = Read::from(world.read_resource::<EntitiesRes>());
        let map_assets = Read::from(world.read_resource::<AssetStorage<Map>>());
        Self::spawn_system(
            &MapSpawningResources {
                entities,
                map_assets,
            },
            &mut MapLayerComponentStorages {
                transparents: world.write_storage::<Transparent>(),
                transforms: world.write_storage::<Transform>(),
                sprite_renders: world.write_storage::<SpriteRender>(),
                component_sequences_handles: world.write_storage::<ComponentSequencesHandle>(),
            },
            map_handle,
        )
    }

    /// Spawns entities for each of the layers in a map.
    ///
    /// # Parameters
    ///
    /// * `map_spawning_resources`: Resources to construct the map with.
    /// * `map_layer_component_storages`: Component storages for the spawned entities.
    /// * `map_handle`: Handle of the map whose layers to spawn.
    pub fn spawn_system<'res, 's>(
        MapSpawningResources {
            entities,
            map_assets,
        }: &MapSpawningResources<'res>,
        MapLayerComponentStorages {
            ref mut transparents,
            ref mut transforms,
            ref mut sprite_renders,
            ref mut component_sequences_handles,
        }: &mut MapLayerComponentStorages<'s>,
        map_handle: &MapHandle,
    ) -> Vec<Entity> {
        let components = {
            let map = map_assets
                .get(map_handle)
                .expect("Expected map to be loaded.");

            // Spawn map layer entities
            if let (Some(sprite_sheet_handles), Some(component_sequences_handles)) =
                (&map.sprite_sheet_handles, &map.component_sequences_handles)
            {
                let components = map
                    .definition
                    .layers
                    .iter()
                    .zip(component_sequences_handles.iter())
                    .filter_map(|(layer, component_sequences_handles)| {
                        // This only spawns an entity if the layer specifies a frame.
                        // In the future it should spawn an entity for shape-based layers.
                        layer.frames.iter().next().map(|frame| {
                            let sheet = frame.sprite.sheet;
                            let sprite_sheet_handle =
                                sprite_sheet_handles.get(sheet).unwrap_or_else(|| {
                                    panic!("Map layer contained invalid sheet number: `{}`", sheet)
                                });
                            let position = layer.position;
                            let mut transform = Transform::default();
                            transform.set_position(Vector3::new(
                                position.x as f32,
                                (position.y - position.z) as f32,
                                position.z as f32,
                            ));

                            let sprite_render = SpriteRender {
                                sprite_sheet: sprite_sheet_handle.clone(),
                                sprite_number: frame.sprite.index,
                            };

                            (
                                transform,
                                sprite_render.clone(),
                                component_sequences_handles.clone(),
                            )
                        })
                    })
                    .collect::<Vec<(Transform, SpriteRender, ComponentSequencesHandle)>>();

                Some(components)

            // kcov-ignore-start
            } else {
                // kcov-ignore-end
                None
            }
        };

        if let Some(layers_entity_components) = components {
            let entities = layers_entity_components
                .into_iter()
                .map(|(transform, sprite_render, component_sequences_handle)| {
                    let entity = entities.create();

                    // Enable transparency for visibility sorting
                    transparents
                        .insert(entity, Transparent)
                        .expect("Failed to insert transparent component.");
                    transforms
                        .insert(entity, transform)
                        .expect("Failed to insert transform component.");
                    sprite_renders
                        .insert(entity, sprite_render)
                        .expect("Failed to insert sprite_render component.");
                    component_sequences_handles
                        .insert(entity, component_sequences_handle)
                        .expect("Failed to insert component_sequences_handle component.");

                    entity
                })
                .collect::<Vec<_>>();

            entities

        // kcov-ignore-start
        } else {
            // kcov-ignore-end
            vec![]
        }
    }
}
