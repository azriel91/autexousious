use std::marker::PhantomData;

use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Entity, Join, Read, ReadStorage, Resources,
        System, SystemData, WriteStorage,
    },
    shrev::ReaderId,
};
use game_loading::{AnimationRunner, ObjectAnimationStorages};
use named_type::NamedType;
use object_model::{
    config::object::SequenceId,
    entity::ObjectStatus,
    loaded::{AnimatedComponentAnimation, Object, ObjectHandle},
};

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
    object_status_modifications: BitSet,
    /// PhantomData.
    phantom_data: PhantomData<SeqId>,
}

type ObjectAnimationUpdateSystemData<'s, SeqId> = (
    Entities<'s>,
    ReadStorage<'s, ObjectStatus<SeqId>>,
    ReadStorage<'s, ObjectHandle<SeqId>>,
    Read<'s, AssetStorage<Object<SeqId>>>,
    ObjectAnimationStorages<'s, SeqId>,
);

impl<SeqId> ObjectAnimationUpdateSystem<SeqId>
where
    SeqId: SequenceId + 'static,
{
    fn swap_animation(
        object: &Object<SeqId>,
        (ref mut sprite_acs, ref mut body_frame_acs, ref mut interaction_acs): &mut ObjectAnimationStorages<
            SeqId,
        >,
        entity: &Entity,
        object_status: &ObjectStatus<SeqId>,
    ) {
        let mut sprite_animation_set = get_animation_set(sprite_acs, *entity)
            .expect("Sprite animation should exist as entity should be valid.");
        let mut body_animation_set = get_animation_set(body_frame_acs, *entity)
            .expect("Body animation should exist as entity should be valid.");
        let mut interaction_animation_set = get_animation_set(interaction_acs, *entity)
            .expect("Interaction animation should exist as entity should be valid.");

        // TODO: replace with actual
        // 1. Update `LastTrackerSystem<T> to take in associated type for what it is tracking from T.
        // 2. Track `SequenceId` from `ObjectStatus`.
        // 3. Add LastTrackerSystem that runs after this one.
        // 4. This system reads `Last<ObjectStatus, Tracked = SeqId>` for this value.
        let last_sequence_id = object_status.sequence_id;
        let next_sequence_id = object_status.sequence_id;

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
        (
            entities,
            object_statuses,
            object_handles,
            object_assets,
            mut object_animation_storages,
        ): Self::SystemData,
    ) {
        // Split borrow self
        let object_status_modifications = &mut self.object_status_modifications;
        let component_ev_rid = self
            .component_ev_rid
            .as_mut()
            .expect("Expected reader ID to exist for ObjectAnimationUpdateSystem.");

        object_status_modifications.clear();
        object_statuses
            .channel()
            .read(component_ev_rid)
            .for_each(|ev| match ev {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    object_status_modifications.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        {
            // Immutable borrow
            let object_status_modifications = &*object_status_modifications;

            (
                &entities,
                &object_statuses,
                &object_handles,
                object_status_modifications,
            )
                .join()
                .for_each(|(entity, object_status, object_handle, _)| {
                    let object = object_assets
                        .get(object_handle)
                        .expect("Expected object to be loaded.");

                    Self::swap_animation(
                        &object,
                        &mut object_animation_storages,
                        &entity,
                        object_status,
                    );
                });
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        let mut object_statuses = WriteStorage::<ObjectStatus<SeqId>>::fetch(res);
        self.component_ev_rid = Some(object_statuses.register_reader());
    }
}
