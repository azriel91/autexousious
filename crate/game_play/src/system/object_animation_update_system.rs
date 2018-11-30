use std::marker::PhantomData;

use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    ecs::{
        storage::ComponentEvent, BitSet, Component, Entities, Entity, Join, Read, ReadStorage,
        Resources, System, SystemData, Tracked, WriteStorage,
    },
    shrev::ReaderId,
};
use game_loading::{AnimationRunner, ObjectAnimationStorages};
use named_type::NamedType;
use object_model::{
    config::object::SequenceId,
    loaded::{AnimatedComponentAnimation, Object, ObjectHandle},
};
use tracker::Last;

/// Updates sequence animations for objects.
///
/// # Type Parameters
///
/// * `ObTy`: Loaded object model type, e.g. `Character`.
/// * `SeqId`: Sequence ID type, e.g. `CharacterSequenceId`.
#[derive(Debug, Default, NamedType, new)]
pub(crate) struct ObjectAnimationUpdateSystem<SeqId> {
    /// Reader ID for the `ComponentEvent` event channel.
    #[new(default)]
    component_ev_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track object status modifications.
    #[new(default)]
    sequence_id_modifications: BitSet,
    /// PhantomData.
    phantom_data: PhantomData<SeqId>,
}

type ObjectAnimationUpdateSystemData<'s, SeqId> = (
    Entities<'s>,
    ReadStorage<'s, Last<SeqId>>,
    ReadStorage<'s, SeqId>,
    ReadStorage<'s, ObjectHandle<SeqId>>,
    Read<'s, AssetStorage<Object<SeqId>>>,
    ObjectAnimationStorages<'s, SeqId>,
);

impl<SeqId> ObjectAnimationUpdateSystem<SeqId>
where
    SeqId: SequenceId + 'static,
    <SeqId as Component>::Storage: Tracked,
{
    fn swap_animation(
        object: &Object<SeqId>,
        (ref mut sprite_acs, ref mut body_frame_acs, ref mut interaction_acs): &mut ObjectAnimationStorages<
            SeqId,
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
    <SeqId as Component>::Storage: Tracked,
{
    type SystemData = ObjectAnimationUpdateSystemData<'s, SeqId>;

    fn run(
        &mut self,
        (
            entities,
            last_sequence_ids,
            sequence_ids,
            object_handles,
            object_assets,
            mut object_animation_storages,
        ): Self::SystemData,
    ) {
        // Split borrow self
        let sequence_id_modifications = &mut self.sequence_id_modifications;
        let component_ev_rid = self
            .component_ev_rid
            .as_mut()
            .expect("Expected reader ID to exist for ObjectAnimationUpdateSystem.");

        sequence_id_modifications.clear();
        sequence_ids
            .channel()
            .read(component_ev_rid)
            .for_each(|ev| match ev {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    sequence_id_modifications.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        {
            // Immutable borrow
            let sequence_id_modifications = &*sequence_id_modifications;

            (
                &entities,
                &last_sequence_ids,
                &sequence_ids,
                &object_handles,
                sequence_id_modifications,
            )
                .join()
                .for_each(
                    |(entity, last_sequence_id, sequence_id, object_handle, _)| {
                        let object = object_assets
                            .get(object_handle)
                            .expect("Expected object to be loaded.");

                        Self::swap_animation(
                            &object,
                            &mut object_animation_storages,
                            &entity,
                            **last_sequence_id,
                            *sequence_id,
                        );
                    },
                );
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        let mut sequence_ids = WriteStorage::<SeqId>::fetch(res);
        self.component_ev_rid = Some(sequence_ids.register_reader());
    }
}
