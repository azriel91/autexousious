use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entity, World, WorldExt},
    renderer::transparent::Transparent,
    shred::SystemData,
};
use asset_model::loaded::{AssetId, AssetIdMappings};
use log::debug;
use num_traits::FromPrimitive;
use sequence_model::{
    loaded::{SequenceEndTransition, SequenceId, WaitSequence},
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use sequence_model_spi::loaded::ComponentDataExt;
use sprite_model::loaded::SpriteRenderSequence;

use crate::{SpriteSequenceComponentStorages, SpriteSequenceSpawningResources};

/// Spawns sprite sequence entities into the world.
#[derive(Debug)]
pub struct SpriteSequenceEntitySpawner;

impl SpriteSequenceEntitySpawner {
    /// Spawns entities for each of the sprite sequences of an asset.
    ///
    /// Idea: What if we could spawn two maps at the same time?
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to spawn the map into.
    /// * `asset_id`: Asset ID of the sprite sequences.
    pub fn spawn_world(world: &mut World, asset_id: AssetId) -> Vec<Entity> {
        // Hack: Need to move all systems into main dispatcher in order to not do this.
        SpriteSequenceSpawningResources::setup(world);
        SpriteSequenceComponentStorages::setup(world);

        {
            let asset_id_mappings = world.read_resource::<AssetIdMappings>();
            let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
                panic!(
                    "Expected asset slug to exist for asset ID: `{:?}`",
                    asset_id,
                )
            });
            debug!("Spawning sprite sequences for asset: `{}`.", asset_slug);
        }

        Self::spawn_system(
            &SpriteSequenceSpawningResources::fetch(&world),
            &mut SpriteSequenceComponentStorages::fetch(&world),
            asset_id,
        )
    }

    /// Spawns entities for each of the sprite sequences of an asset.
    ///
    /// # Parameters
    ///
    /// * `map_spawning_resources`: Resources to construct the map with.
    /// * `map_sprite_sequence_component_storages`: Component storages for the spawned entities.
    /// * `asset_id`: Asset ID of the sprite sequences.
    pub fn spawn_system<'res, 's>(
        SpriteSequenceSpawningResources {
            entities,
            asset_wait_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_sprite_positions,
            wait_sequence_assets,
            sprite_render_sequence_assets,
        }: &SpriteSequenceSpawningResources<'res>,
        SpriteSequenceComponentStorages {
            transparents,
            transforms,
            waits,
            sequence_ids,
            sequence_end_transitions,
            sequence_statuses,
            frame_index_clocks,
            frame_wait_clocks,
            sprite_renders,
            wait_sequence_handles,
            sprite_render_sequence_handles,
        }: &mut SpriteSequenceComponentStorages<'s>,
        asset_id: AssetId,
    ) -> Vec<Entity> {
        let asset_wait_sequence_handles = asset_wait_sequence_handles.get(asset_id);
        let asset_sprite_render_sequence_handles =
            asset_sprite_render_sequence_handles.get(asset_id);
        let sprite_positions = asset_sprite_positions.get(asset_id);

        // Spawn sprite sequence entities
        if let (
            Some(asset_wait_sequence_handles),
            Some(asset_sprite_render_sequence_handles),
            Some(sprite_positions),
        ) = (
            asset_wait_sequence_handles,
            asset_sprite_render_sequence_handles,
            sprite_positions,
        ) {
            sprite_positions
                .iter()
                .zip(asset_wait_sequence_handles.iter())
                .zip(asset_sprite_render_sequence_handles.iter())
                .map(
                    |((sprite_position, wait_sequence_handle), sprite_render_sequence_handle)| {
                        let entity = entities.create();

                        let mut transform = Transform::default();
                        transform.set_translation(Vector3::new(
                            f32::from_i32(sprite_position.x)
                                .expect("Failed to convert i32 into `f32`."),
                            f32::from_i32(sprite_position.y - sprite_position.z)
                                .expect("Failed to convert i32 into `f32`."),
                            f32::from_i32(sprite_position.z)
                                .expect("Failed to convert i32 into `f32`."),
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
                        sequence_ids
                            .insert(entity, SequenceId::default())
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
        } else {
            Vec::new()
        }
    }
}
