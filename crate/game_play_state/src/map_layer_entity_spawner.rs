use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    renderer::{Material, MeshHandle},
};
use map_model::loaded::{Map, MapHandle};

use AnimationRunner;

/// Spawns map layer entities into the world.
#[derive(Debug)]
pub(crate) struct MapLayerEntitySpawner;

impl MapLayerEntitySpawner {
    /// Spawns entities for each of the layers in a map.
    ///
    /// Idea: What if we could spawn two maps at the same time?
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to spawn the character into.
    /// * `map_handle`: Handle of the map whose layers to spawn.
    pub(crate) fn spawn(world: &mut World, map_handle: &MapHandle) -> Vec<Entity> {
        let components_and_animation = {
            let map_store = world.read_resource::<AssetStorage<Map>>();
            let map = map_store
                .get(map_handle)
                .expect("Expected map to be loaded.");

            // Spawn map layer entities
            let map_animations = &map.animations;
            if map_animations.is_some() {
                let sprite_material_mesh = map
                    .sprite_material_mesh
                    .as_ref()
                    .expect("Expected SpriteMaterialMesh to be present when there are animations.");

                let map_animations = map_animations.as_ref().unwrap().clone();

                let components = map
                    .definition
                    .layers
                    .iter()
                    .map(|layer| {
                        let position = layer.position;
                        let mut transform = Transform::default();
                        transform.translation =
                            Vector3::new(position.x as f32, position.y as f32, position.z as f32);

                        (
                            transform,
                            sprite_material_mesh.default_material.clone(),
                            sprite_material_mesh.mesh.clone(),
                        )
                    })
                    .collect::<Vec<(Transform, Material, MeshHandle)>>();

                Some((components, map_animations))
            } else {
                None
            }
        };

        if let Some((layers_entity_components, map_animations)) = components_and_animation {
            let entities = layers_entity_components
                .into_iter()
                .map(|(transform, material, mesh)| {
                    world
                        .create_entity()
                        .with(transform)
                        .with(material)
                        .with(mesh)
                        .with(GlobalTransform::default())
                        .build()
                })
                .collect::<Vec<_>>();

            entities
                .iter()
                .zip(map_animations.iter())
                .for_each(|(entity, animation_handle)| {
                    // We also need to trigger the animation, not just attach it to the entity
                    let animation_id = 0;
                    let mut animation_control_set_storage = world.write_storage();
                    let mut animation_set =
                        get_animation_set::<u32, Material>(
                            &mut animation_control_set_storage,
                            *entity,
                        ).expect("Animation should exist as new entity should be valid.");

                    AnimationRunner::start_loop(&mut animation_set, animation_handle, animation_id);
                });

            entities
        } else {
            vec![]
        }
    }
}
