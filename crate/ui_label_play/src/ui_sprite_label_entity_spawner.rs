use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entity, World, WorldExt},
    renderer::transparent::Transparent,
    shred::SystemData,
};
use asset_model::loaded::{AssetId, AssetIdMappings};
use kinematic_model::config::Position;
use log::debug;
use sequence_model::{
    loaded::WaitSequence,
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use sequence_model_spi::loaded::ComponentDataExt;
use sprite_model::loaded::SpriteRenderSequence;

use crate::{UiSpriteLabelComponentStorages, UiSpriteLabelSpawningResources};

/// Spawns sprite sequence entities into the world.
#[derive(Debug)]
pub struct UiSpriteLabelEntitySpawner;

impl UiSpriteLabelEntitySpawner {
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
        UiSpriteLabelSpawningResources::setup(world);
        UiSpriteLabelComponentStorages::setup(world);

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
            &UiSpriteLabelSpawningResources::fetch(&world),
            &mut UiSpriteLabelComponentStorages::fetch(&world),
            asset_id,
        )
    }

    /// Spawns entities for each of the sprite sequences of an asset.
    ///
    /// # Parameters
    ///
    /// * `ui_sprite_label_spawning_resources`: Resources to construct the map with.
    /// * `ui_sprite_label_component_storages`: Component storages for the spawned entities.
    /// * `asset_id`: Asset ID of the sprite sequences.
    pub fn spawn_system<'res, 's>(
        UiSpriteLabelSpawningResources {
            entities,
            asset_ui_sprite_labels,
            asset_wait_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_sequence_end_transitions,
            wait_sequence_assets,
            sprite_render_sequence_assets,
        }: &UiSpriteLabelSpawningResources<'res>,
        UiSpriteLabelComponentStorages {
            asset_ids,
            transparents,
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
        }: &mut UiSpriteLabelComponentStorages<'s>,
        asset_id: AssetId,
    ) -> Vec<Entity> {
        let asset_ui_sprite_labels = asset_ui_sprite_labels.get(asset_id);
        let asset_wait_sequence_handles = asset_wait_sequence_handles.get(asset_id);
        let asset_sprite_render_sequence_handles =
            asset_sprite_render_sequence_handles.get(asset_id);
        let asset_sequence_end_transitions = asset_sequence_end_transitions.get(asset_id);

        // Spawn sprite sequence entities
        if let (
            Some(asset_ui_sprite_labels),
            Some(asset_wait_sequence_handles),
            Some(asset_sprite_render_sequence_handles),
            Some(asset_sequence_end_transitions),
        ) = (
            asset_ui_sprite_labels,
            asset_wait_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_sequence_end_transitions,
        ) {
            asset_ui_sprite_labels
                .iter()
                .map(|ui_sprite_label| {
                    let sequence_id = ui_sprite_label.sequence_id;
                    let wait_sequence_handle = asset_wait_sequence_handles
                        .get(*sequence_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "Expected `WaitSequenceHandle` to exist for sequence ID: `{:?}`",
                                sequence_id
                            )
                        });
                    let sprite_render_sequence_handle = asset_sprite_render_sequence_handles
                        .get(*sequence_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "Expected `SpriteRenderSequenceHandle` to exist for \
                                 sequence ID: `{:?}`",
                                sequence_id
                            )
                        });
                    let sequence_end_transition = asset_sequence_end_transitions
                        .get(*sequence_id)
                        .copied()
                        .unwrap_or_else(|| {
                            panic!(
                                "Expected `SequenceEndTransition` to exist for sequence ID: `{:?}`",
                                sequence_id
                            )
                        });

                    let entity = entities.create();

                    let translation = Into::<Vector3<f32>>::into(ui_sprite_label.position);
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
                    positions
                        .insert(entity, position)
                        .expect("Failed to insert `Position<f32>` component.");
                    transforms
                        .insert(entity, transform)
                        .expect("Failed to insert `Transform` component.");
                    sequence_ids
                        .insert(entity, sequence_id)
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
                })
                .collect::<Vec<Entity>>()
        } else {
            Vec::new()
        }
    }
}
