use std::marker::PhantomData;

use amethyst::{
    animation::get_animation_set,
    assets::{AssetStorage, Handle},
    ecs::{Entities, Entity, Join, Read, ReadStorage, System},
};
use derive_new::new;
use game_loading::{AnimationRunner, ObjectAnimationStorages};
use named_type::NamedType;
use named_type_derive::NamedType;
use object_model::{
    entity::SequenceStatus,
    loaded::{AnimatedComponentAnimation, GameObject, ObjectWrapper},
};
use shred_derive::SystemData;
use tracker::Last;

/// Updates sequence animations for objects.
///
/// # Type Parameters
///
/// * `O`: `GameObject` type, e.g. `Character`.
#[derive(Debug, Default, NamedType, new)]
pub(crate) struct ObjectAnimationUpdateSystem<O> {
    /// PhantomData.
    phantom_data: PhantomData<O>,
}

#[allow(missing_debug_implementations)]
#[derive(SystemData)]
pub struct ObjectAnimationUpdateSystemData<'s, O>
where
    O: GameObject,
{
    entities: Entities<'s>,
    sequence_statuses: ReadStorage<'s, SequenceStatus>,
    last_sequence_ids: ReadStorage<'s, Last<O::SequenceId>>,
    sequence_ids: ReadStorage<'s, O::SequenceId>,
    object_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    object_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    object_acses: ObjectAnimationStorages<'s, O::SequenceId>,
}

impl<O> ObjectAnimationUpdateSystem<O>
where
    O: GameObject,
{
    fn swap_animation(
        object: &O::ObjectWrapper,
        ObjectAnimationStorages {
            ref mut sprite_render_acses,
            ref mut body_acses,
            ref mut interaction_acses,
        }: &mut ObjectAnimationStorages<'_, O::SequenceId>,
        entity: Entity,
        last_sequence_id: O::SequenceId,
        next_sequence_id: O::SequenceId,
    ) {
        let mut sprite_animation_set = get_animation_set(sprite_render_acses, entity)
            .expect("Sprite animation should exist as entity should be valid.");
        let mut body_animation_set = get_animation_set(body_acses, entity)
            .expect("Body animation should exist as entity should be valid.");
        let mut interaction_animation_set = get_animation_set(interaction_acses, entity)
            .expect("Interaction animation should exist as entity should be valid.");

        let animations = object
            .inner()
            .animations
            .get(&next_sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get animations for sequence: `{:?}`",
                    next_sequence_id
                )
            });

        animations
            .iter()
            .for_each(|animated_component| match animated_component {
                AnimatedComponentAnimation::SpriteRender(ref handle) => {
                    AnimationRunner::swap(
                        last_sequence_id,
                        next_sequence_id,
                        &mut sprite_animation_set,
                        handle,
                    );
                }
                AnimatedComponentAnimation::BodyFrame(ref handle) => {
                    AnimationRunner::swap(
                        last_sequence_id,
                        next_sequence_id,
                        &mut body_animation_set,
                        handle,
                    );
                }
                AnimatedComponentAnimation::InteractionFrame(ref handle) => {
                    AnimationRunner::swap(
                        last_sequence_id,
                        next_sequence_id,
                        &mut interaction_animation_set,
                        handle,
                    );
                }
            });
    }
}

impl<'s, O> System<'s> for ObjectAnimationUpdateSystem<O>
where
    O: GameObject,
{
    type SystemData = ObjectAnimationUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        ObjectAnimationUpdateSystemData {
            entities,
            sequence_statuses,
            last_sequence_ids,
            sequence_ids,
            object_handles,
            object_assets,
            mut object_acses,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &sequence_statuses,
            &last_sequence_ids,
            &sequence_ids,
            &object_handles,
        )
            .join()
            .filter(|(_, sequence_status, _, _, _)| **sequence_status == SequenceStatus::Begin)
            .for_each(
                |(entity, _, last_sequence_id, sequence_id, object_handle)| {
                    let object = object_assets
                        .get(object_handle)
                        .expect("Expected object to be loaded.");

                    Self::swap_animation(
                        &object,
                        &mut object_acses,
                        entity,
                        **last_sequence_id,
                        *sequence_id,
                    );
                },
            );
    }
}
