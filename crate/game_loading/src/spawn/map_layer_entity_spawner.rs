use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entity, SystemData, World},
    renderer::Transparent,
};
use logic_clock::LogicClock;
use map_model::loaded::MapHandle;
use sequence_model::{
    config::Repeat,
    entity::{FrameIndexClock, SequenceStatus},
    loaded::ComponentSequence,
};

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
        Self::spawn_system(
            &MapSpawningResources::fetch(&world.res),
            &mut MapLayerComponentStorages::fetch(&world.res),
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
            component_sequences_assets,
        }: &MapSpawningResources<'res>,
        MapLayerComponentStorages {
            ref mut transparents,
            ref mut transforms,
            ref mut waits,
            ref mut repeats,
            ref mut sequence_statuses,
            ref mut frame_index_clocks,
            ref mut logic_clocks,
            ref mut sprite_renders,
            ref mut component_sequences_handles,
        }: &mut MapLayerComponentStorages<'s>,
        map_handle: &MapHandle,
    ) -> Vec<Entity> {
        let map = map_assets
            .get(map_handle)
            .expect("Expected map to be loaded.");

        // Spawn map layer entities
        if let Some(map_comp_seq_handles) = &map.component_sequences_handles {
            map.definition
                .layers
                .iter()
                .zip(map_comp_seq_handles.iter())
                .map(|(layer, component_sequences_handle)| {
                    let entity = entities.create();

                    let position = layer.position;
                    let mut transform = Transform::default();
                    transform.set_translation(Vector3::new(
                        position.x as f32,
                        (position.y - position.z) as f32,
                        position.z as f32,
                    ));

                    let component_sequences = component_sequences_assets
                        .get(component_sequences_handle)
                        .expect("Expected `ComponentSequences` to be loaded.");

                    let frame_index_clock =
                        FrameIndexClock::new(LogicClock::new(component_sequences.frame_count()));
                    frame_index_clocks
                        .insert(entity, frame_index_clock)
                        .expect("Failed to insert frame_index_clock component.");
                    let starting_frame_index = (*frame_index_clock).value;
                    let mut logic_clock = LogicClock::new(1);

                    component_sequences.iter().for_each(|component_sequence| {
                        match component_sequence {
                            ComponentSequence::Wait(wait_sequence) => {
                                let wait = wait_sequence[starting_frame_index];
                                waits
                                    .insert(entity, wait)
                                    .expect("Failed to insert `Wait` component for object.");

                                logic_clock.limit = *wait as usize;
                            }
                            ComponentSequence::SpriteRender(sprite_render_sequence) => {
                                let sprite_render =
                                    sprite_render_sequence[starting_frame_index].clone();
                                sprite_renders.insert(entity, sprite_render).expect(
                                    "Failed to insert `SpriteRender` component for object.",
                                );
                            }
                            _ => {} // do nothing
                        }
                    });

                    // Enable transparency for visibility sorting
                    transparents
                        .insert(entity, Transparent)
                        .expect("Failed to insert transparent component.");
                    transforms
                        .insert(entity, transform)
                        .expect("Failed to insert transform component.");
                    repeats
                        .insert(entity, Repeat)
                        .expect("Failed to insert repeat component.");
                    sequence_statuses
                        .insert(entity, SequenceStatus::default())
                        .expect("Failed to insert sequence_status component.");
                    component_sequences_handles
                        .insert(entity, component_sequences_handle.clone())
                        .expect("Failed to insert component_sequences_handle component.");
                    logic_clocks
                        .insert(entity, logic_clock)
                        .expect("Failed to insert logic_clock component.");

                    entity
                })
                .collect::<Vec<Entity>>()
        // kcov-ignore-start
        } else {
            // kcov-ignore-end
            vec![]
        }
    }
}
