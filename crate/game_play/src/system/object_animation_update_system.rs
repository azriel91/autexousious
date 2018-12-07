use std::marker::PhantomData;

use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System},
};
use game_loading::{AnimationRunner, ObjectAnimationStorages};
use named_type::NamedType;
use object_model::{
    config::object::SequenceId,
    entity::SequenceStatus,
    loaded::{AnimatedComponentAnimation, Object, ObjectHandle},
};
use shred_derive::SystemData;
use tracker::Last;

/// Updates sequence animations for objects.
///
/// # Type Parameters
///
/// * `SeqId`: Sequence ID type, e.g. `CharacterSequenceId`.
#[derive(Debug, Default, NamedType, new)]
pub(crate) struct ObjectAnimationUpdateSystem<SeqId> {
    /// PhantomData.
    phantom_data: PhantomData<SeqId>,
}

#[allow(missing_debug_implementations)]
#[derive(SystemData)]
pub struct ObjectAnimationUpdateSystemData<'s, SeqId>
where
    SeqId: SequenceId + 'static,
{
    entities: Entities<'s>,
    sequence_statuses: ReadStorage<'s, SequenceStatus>,
    last_sequence_ids: ReadStorage<'s, Last<SeqId>>,
    sequence_ids: ReadStorage<'s, SeqId>,
    object_handles: ReadStorage<'s, ObjectHandle<SeqId>>,
    object_assets: Read<'s, AssetStorage<Object<SeqId>>>,
    object_acses: ObjectAnimationStorages<'s, SeqId>,
}

impl<SeqId> ObjectAnimationUpdateSystem<SeqId>
where
    SeqId: SequenceId + 'static,
{
    fn swap_animation(
        object: &Object<SeqId>,
        (ref mut sprite_acs, ref mut body_frame_acs, ref mut interaction_acs): &mut ObjectAnimationStorages<
            '_, SeqId,
        >,
        entity: &Entity,
        last_sequence_id: SeqId,
        next_sequence_id: SeqId,
    ) {
        let mut sprite_animation_set = get_animation_set(sprite_acs, *entity)
            .expect("Sprite animation should exist as entity should be valid.");
        let mut body_animation_set = get_animation_set(body_frame_acs, *entity)
            .expect("Body animation should exist as entity should be valid.");
        let mut interaction_animation_set = get_animation_set(interaction_acs, *entity)
            .expect("Interaction animation should exist as entity should be valid.");

        let animations = &object.animations.get(&next_sequence_id).unwrap_or_else(|| {
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

impl<'s, SeqId> System<'s> for ObjectAnimationUpdateSystem<SeqId>
where
    SeqId: SequenceId + 'static,
{
    type SystemData = ObjectAnimationUpdateSystemData<'s, SeqId>;

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
                        &entity,
                        **last_sequence_id,
                        *sequence_id,
                    );
                },
            );
    }
}
