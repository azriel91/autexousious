use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entity, World},
    renderer::transparent::Transparent,
    shred::SystemData,
};
use map_model::{config::MapLayerSequenceId, loaded::MapHandle};
use num_traits::FromPrimitive;
use sequence_model::{
    loaded::{SequenceEndTransition, WaitSequence},
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use sequence_model_spi::loaded::ComponentDataExt;
use sprite_model::loaded::SpriteRenderSequence;

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
            &MapSpawningResources::fetch(&world),
            &mut MapLayerComponentStorages::fetch(&world),
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
            wait_sequence_assets,
            sprite_render_sequence_assets,
        }: &MapSpawningResources<'res>,
        MapLayerComponentStorages {
            ref mut transparents,
            ref mut transforms,
            ref mut waits,
            ref mut map_layer_sequence_ids,
            ref mut sequence_end_transitions,
            ref mut sequence_statuses,
            ref mut frame_index_clocks,
            ref mut frame_wait_clocks,
            ref mut sprite_renders,
            ref mut wait_sequence_handles,
            ref mut sprite_render_sequence_handles,
        }: &mut MapLayerComponentStorages<'s>,
        map_handle: &MapHandle,
    ) -> Vec<Entity> {
        let map = map_assets
            .get(map_handle)
            .expect("Expected map to be loaded.");

        // Spawn map layer entities
        map.definition
            .layers
            .iter()
            .zip(map.wait_sequence_handles.iter())
            .zip(map.sprite_render_sequence_handles.iter())
            .map(
                |((layer, wait_sequence_handle), sprite_render_sequence_handle)| {
                    let entity = entities.create();

                    let position = layer.position;
                    let mut transform = Transform::default();
                    transform.set_translation(Vector3::new(
                        f32::from_i32(position.x).expect("Failed to convert i32 into `f32`."),
                        f32::from_i32(position.y - position.z)
                            .expect("Failed to convert i32 into `f32`."),
                        f32::from_i32(position.z).expect("Failed to convert i32 into `f32`."),
                    ));

                    let starting_frame_index = 0;
                    let wait_sequence = wait_sequence_assets
                        .get(wait_sequence_handle)
                        .expect("Expected `WaitSequence` to be loaded.");
                    let wait = <WaitSequence as ComponentDataExt>::to_owned(
                        &wait_sequence[starting_frame_index],
                    );
                    waits
                        .insert(entity, wait)
                        .expect("Failed to insert `Wait` component.");

                    let sprite_render_sequence = sprite_render_sequence_assets
                        .get(sprite_render_sequence_handle)
                        .expect("Expected `SpriteRenderSequence` to be loaded.");
                    let sprite_render = <SpriteRenderSequence as ComponentDataExt>::to_owned(
                        &sprite_render_sequence[starting_frame_index],
                    );
                    sprite_renders
                        .insert(entity, sprite_render)
                        .expect("Failed to insert `SpriteRender` component.");

                    let frame_index_clock = FrameIndexClock::new(wait_sequence.len());
                    frame_index_clocks
                        .insert(entity, frame_index_clock)
                        .expect("Failed to insert frame_index_clock component.");

                    let frame_wait_clock = FrameWaitClock::new(*wait as usize);
                    frame_wait_clocks
                        .insert(entity, frame_wait_clock)
                        .expect("Failed to insert frame_wait_clock component.");

                    // Enable transparency for visibility sorting
                    transparents
                        .insert(entity, Transparent)
                        .expect("Failed to insert transparent component.");
                    transforms
                        .insert(entity, transform)
                        .expect("Failed to insert transform component.");
                    map_layer_sequence_ids
                        .insert(entity, MapLayerSequenceId::default())
                        .expect("Failed to insert sequence_end_transition component.");
                    sequence_end_transitions
                        .insert(entity, SequenceEndTransition::Repeat)
                        .expect("Failed to insert sequence_end_transition component.");
                    sequence_statuses
                        .insert(entity, SequenceStatus::default())
                        .expect("Failed to insert sequence_status component.");
                    wait_sequence_handles
                        .insert(entity, wait_sequence_handle.clone())
                        .expect("Failed to insert wait_sequence_handle component.");
                    sprite_render_sequence_handles
                        .insert(entity, sprite_render_sequence_handle.clone())
                        .expect("Failed to insert sprite_render_sequence_handle component.");

                    entity
                },
            )
            .collect::<Vec<Entity>>()
    }
}
