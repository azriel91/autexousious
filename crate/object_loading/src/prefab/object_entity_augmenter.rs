use amethyst::{
    animation::get_animation_set,
    core::transform::Transform,
    ecs::Entity,
    renderer::{Flipped, SpriteRender, Transparent},
};
use animation_support::AnimationRunner;
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use object_model::{
    entity::{Mirrored, Position, SequenceStatus, Velocity},
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault, ObjectWrapper},
};

use crate::{ObjectAnimationStorages, ObjectComponentStorages};

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `object_component_storages`: Common `Component` storages for objects.
    /// * `object_animation_storages`: Common animation storages for objects.
    /// * `object_wrapper`: Slug and handle of the object to spawn.
    pub fn augment<'s, W>(
        entity: Entity,
        ObjectComponentStorages {
            ref mut sprite_renders,
            ref mut flippeds,
            ref mut transparents,
            ref mut positions,
            ref mut velocities,
            ref mut transforms,
            ref mut mirroreds,
            ref mut sequence_end_transitionses,
            ref mut sequence_ids,
            ref mut sequence_statuses,
            ref mut body_frame_active_handles,
            ref mut interaction_frame_active_handles,
        }: &mut ObjectComponentStorages<'s, W::SequenceId>,
        ObjectAnimationStorages {
            ref mut sprite_render_acses,
            ref mut body_acses,
            ref mut interaction_acses,
        }: &mut ObjectAnimationStorages<'s, W::SequenceId>,
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

        let all_animations = object_wrapper.inner().animations.get(&sequence_id);
        let animation_defaults = &object_wrapper.inner().animation_defaults;
        let first_sequence_animations = all_animations
            .as_ref()
            .expect("Expected game object to have at least one sequence.");

        animation_defaults
            .iter()
            .for_each(|animation_default| match animation_default {
                AnimatedComponentDefault::SpriteRender(ref sprite_render) => {
                    // The starting pose
                    sprite_renders
                        .insert(entity, sprite_render.clone())
                        .expect("Failed to insert `SpriteRender` component.");
                }
                AnimatedComponentDefault::BodyFrame(ref active_handle) => {
                    // Default body active handle
                    body_frame_active_handles
                        .insert(entity, active_handle.clone())
                        .expect("Failed to insert `BodyFrameActiveHandle` component.");
                }
                AnimatedComponentDefault::InteractionFrame(ref active_handle) => {
                    // Default interaction active handle
                    interaction_frame_active_handles
                        .insert(entity, active_handle.clone())
                        .expect("Failed to insert `InteractionFrameActiveHandle` component.");
                }
            });

        // We also need to trigger the animation, not just attach it to the entity
        let mut sprite_animation_set =
            get_animation_set::<W::SequenceId, SpriteRender>(sprite_render_acses, entity)
                .expect("Sprite animation should exist as new entity should be valid.");
        let mut body_animation_set =
            get_animation_set::<W::SequenceId, BodyFrameActiveHandle>(body_acses, entity)
                .expect("Body animation should exist as new entity should be valid.");
        let mut interaction_animation_set = get_animation_set::<
            W::SequenceId,
            InteractionFrameActiveHandle,
        >(interaction_acses, entity)
        .expect("Interaction animation should exist as new entity should be valid.");

        first_sequence_animations
            .iter()
            .for_each(|animated_component| match animated_component {
                AnimatedComponentAnimation::SpriteRender(ref handle) => {
                    AnimationRunner::start(sequence_id, &mut sprite_animation_set, handle);
                }
                AnimatedComponentAnimation::BodyFrame(ref handle) => {
                    AnimationRunner::start(sequence_id, &mut body_animation_set, handle);
                }
                AnimatedComponentAnimation::InteractionFrame(ref handle) => {
                    AnimationRunner::start(sequence_id, &mut interaction_animation_set, handle);
                }
            });
    }
}

// See tests/object_entity_augmenter.rs
