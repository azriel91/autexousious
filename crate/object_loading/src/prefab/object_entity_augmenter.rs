use amethyst::{
    assets::AssetStorage,
    core::transform::Transform,
    ecs::Entity,
    renderer::{Flipped, Transparent},
};
use logic_clock::LogicClock;
use object_model::{
    entity::{FrameIndexClock, Mirrored, Position, SequenceStatus, Velocity},
    loaded::{ComponentSequence, ComponentSequences, ObjectWrapper},
};

use crate::{FrameComponentStorages, ObjectComponentStorages};

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `component_sequences_assets`: Asset storage for `ComponentSequences`.
    /// * `object_component_storages`: Non-frame-dependent `Component` storages for objects.
    /// * `frame_component_storages`: Frame component storages for objects.
    /// * `object_wrapper`: Slug and handle of the object to spawn.
    pub fn augment<'s, W>(
        entity: Entity,
        component_sequences_assets: &AssetStorage<ComponentSequences>,
        ObjectComponentStorages {
            ref mut flippeds,
            ref mut transparents,
            ref mut positions,
            ref mut velocities,
            ref mut transforms,
            ref mut mirroreds,
            ref mut component_sequences_handles,
            ref mut sequence_end_transitionses,
            ref mut sequence_ids,
            ref mut sequence_statuses,
            ref mut frame_index_clocks,
            ref mut logic_clocks,
        }: &mut ObjectComponentStorages<'s, W::SequenceId>,
        FrameComponentStorages {
            ref mut waits,
            ref mut sprite_renders,
            ref mut bodies,
            ref mut interactionses,
        }: &mut FrameComponentStorages<'s>,
        object_wrapper: &W,
    ) where
        W: ObjectWrapper,
    {
        let sequence_end_transitions = &object_wrapper.inner().sequence_end_transitions;

        let sequence_id = W::SequenceId::default();

        flippeds
            .insert(entity, Flipped::None)
            .expect("Failed to insert flipped component.");
        transparents
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        positions
            .insert(entity, Position::default())
            .expect("Failed to insert position component.");
        velocities
            .insert(entity, Velocity::default())
            .expect("Failed to insert velocity component.");
        transforms
            .insert(entity, Transform::default())
            .expect("Failed to insert transform component.");
        mirroreds
            .insert(entity, Mirrored::default())
            .expect("Failed to insert mirrored component.");
        sequence_end_transitionses
            .insert(entity, sequence_end_transitions.clone())
            .expect("Failed to insert sequence_end_transitions component.");
        sequence_ids
            .insert(entity, sequence_id)
            .expect("Failed to insert sequence_id component.");
        sequence_statuses
            .insert(entity, SequenceStatus::default())
            .expect("Failed to insert sequence_status component.");

        let component_sequences_handle = object_wrapper
            .inner()
            .component_sequences_handles
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get `ComponentSequencesHandle` for sequence ID: \
                     `{:?}`.",
                    sequence_id
                );
            });

        let component_sequences = component_sequences_assets
            .get(component_sequences_handle)
            .unwrap_or_else(|| {
                panic!(
                    "Expected component_sequences to be loaded for sequence_id: `{:?}`",
                    sequence_id
                )
            });

        let frame_index_clock =
            FrameIndexClock::new(LogicClock::new(component_sequences.frame_count()));
        let starting_frame_index = (*frame_index_clock).value;
        frame_index_clocks
            .insert(entity, frame_index_clock)
            .expect("Failed to insert frame_index_clock component.");
        let mut logic_clock = LogicClock::new(1);

        component_sequences
            .iter()
            .for_each(|component_sequence| match component_sequence {
                ComponentSequence::Wait(wait_sequence) => {
                    let wait = wait_sequence[starting_frame_index];
                    waits
                        .insert(entity, wait)
                        .expect("Failed to insert `Wait` component for object.");

                    logic_clock.limit = *wait as usize;
                }
                ComponentSequence::SpriteRender(sprite_render_sequence) => {
                    let sprite_render = sprite_render_sequence[starting_frame_index].clone();
                    sprite_renders
                        .insert(entity, sprite_render)
                        .expect("Failed to insert `SpriteRender` component for object.");
                }
                ComponentSequence::Body(body_sequence) => {
                    let body_handle = body_sequence[starting_frame_index].clone();
                    bodies
                        .insert(entity, body_handle)
                        .expect("Failed to insert `Body` component for object.");
                }
                ComponentSequence::Interactions(interactions_sequence) => {
                    let interactions_handle = interactions_sequence[starting_frame_index].clone();
                    interactionses
                        .insert(entity, interactions_handle)
                        .expect("Failed to insert `Interactions` component for object.");
                }
            });

        component_sequences_handles
            .insert(entity, component_sequences_handle.clone())
            .expect("Failed to insert component_sequences_handle component.");

        logic_clocks
            .insert(entity, logic_clock)
            .expect("Failed to insert logic_clock component.");
    }
}

// See tests/object_entity_augmenter.rs
