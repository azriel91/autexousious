use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entity, World, WorldExt},
    renderer::transparent::Transparent,
    shred::SystemData,
};
use asset_model::loaded::{AssetId, AssetIdMappings};
use kinematic_model::config::Position;
use log::debug;
use num_traits::FromPrimitive;
use sequence_model::{
    loaded::{SequenceId, WaitSequence},
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
            asset_sequence_end_transitions,
            wait_sequence_assets,
            sprite_render_sequence_assets,
        }: &SpriteSequenceSpawningResources<'res>,
        SpriteSequenceComponentStorages {
            asset_ids,
            transparents,
            sprite_positions,
            positions,
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
        let asset_sprite_positions = asset_sprite_positions.get(asset_id);
        let asset_sequence_end_transitions = asset_sequence_end_transitions.get(asset_id);

        // Spawn sprite sequence entities
        if let (
            Some(asset_wait_sequence_handles),
            Some(asset_sprite_render_sequence_handles),
            Some(asset_sprite_positions),
            Some(asset_sequence_end_transitions),
        ) = (
            asset_wait_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_sprite_positions,
            asset_sequence_end_transitions,
        ) {
            asset_sprite_positions
                .iter()
                .copied()
                .zip(asset_wait_sequence_handles.iter())
                .zip(asset_sprite_render_sequence_handles.iter())
                .zip(asset_sequence_end_transitions.iter().copied())
                .map(
                    |(
                        ((sprite_position, wait_sequence_handle), sprite_render_sequence_handle),
                        sequence_end_transition,
                    )| {
                        let entity = entities.create();

                        let translation = Vector3::new(
                            f32::from_i32(sprite_position.x)
                                .expect("Failed to convert i32 into `f32`."),
                            f32::from_i32(sprite_position.y)
                                .expect("Failed to convert i32 into `f32`."),
                            f32::from_i32(sprite_position.z)
                                .expect("Failed to convert i32 into `f32`."),
                        );
                        let position = Position::from(translation);
                        let mut transform = Transform::default();
                        transform.set_translation(translation);

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
                        asset_ids
                            .insert(entity, asset_id)
                            .expect("Failed to insert `AssetId` component.");
                        transparents
                            .insert(entity, Transparent)
                            .expect("Failed to insert `Transparent` component.");
                        sprite_positions
                            .insert(entity, sprite_position)
                            .expect("Failed to insert `SpritePosition` component.");
                        positions
                            .insert(entity, position)
                            .expect("Failed to insert `Position<f32>` component.");
                        transforms
                            .insert(entity, transform)
                            .expect("Failed to insert `Transform` component.");
                        sequence_ids
                            .insert(entity, SequenceId::default())
                            .expect("Failed to insert `SequenceEndTransition` component.");
                        sequence_end_transitions
                            .insert(entity, sequence_end_transition)
                            .expect("Failed to insert `SequenceEndTransition` component.");
                        sequence_statuses
                            .insert(entity, SequenceStatus::default())
                            .expect("Failed to insert `SequenceStatus` component.");
                        wait_sequence_handles
                            .insert(entity, wait_sequence_handle.clone())
                            .expect("Failed to insert `WaitSequenceHandle` component.");
                        sprite_render_sequence_handles
                            .insert(entity, sprite_render_sequence_handle.clone())
                            .expect("Failed to insert `SpriteRenderSequenceHandle` component.");

                        entity
                    },
                )
                .collect::<Vec<Entity>>()
        } else {
            Vec::new()
        }
    }
}
