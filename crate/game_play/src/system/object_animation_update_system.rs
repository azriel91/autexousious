use std::marker::PhantomData;

use amethyst::{
    animation::get_animation_set,
    assets::{Asset, AssetStorage, Handle},
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Entity, Read, ReadStorage, Resources, System,
        SystemData, WriteStorage,
    },
    shrev::ReaderId,
};
use game_loading::{AnimationRunner, ObjectAnimationStorages};
use named_type::NamedType;
use object_model::{
    config::object::SequenceId, entity::ObjectStatus, loaded::AnimatedComponentAnimation,
};

/// Updates sequence animations for objects.
///
/// # Type Parameters
///
/// * `ObTy`: Loaded object model type, e.g. `Character`.
/// * `SeqId`: Sequence ID type, e.g. `CharacterSequenceId`.
#[derive(Debug, Default, NamedType, new)]
pub(crate) struct ObjectAnimationUpdateSystem<ObTy, SeqId> {
    /// Reader ID for the `ComponentEvent` event channel.
    #[new(default)]
    component_ev_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track object status modifications.
    #[new(default)]
    object_status_modifications: BitSet,
    /// PhantomData.
    phantom_data: PhantomData<(ObTy, SeqId)>,
}

type ObjectAnimationUpdateSystemData<'s, ObTy, SeqId> = (
    Entities<'s>,
    ReadStorage<'s, ObjectStatus<SeqId>>,
    ReadStorage<'s, Handle<ObTy>>,
    Read<'s, AssetStorage<ObTy>>,
    ObjectAnimationStorages<'s, SeqId>,
);

impl<ObTy, SeqId> ObjectAnimationUpdateSystem<ObTy, SeqId>
where
    ObTy: Asset,
    SeqId: SequenceId + 'static,
{
    fn swap_animation(
        ob_ty_assets: &Read<AssetStorage<ObTy>>,
        (mut sprite_acs, mut body_frame_acs, mut interaction_acs): &mut ObjectAnimationStorages<
            SeqId,
        >,
        entity: &Entity,
        object_status: &ObjectStatus<SeqId>,
        ob_ty_handle: &Handle<ObTy>,
    ) {
        let ob_ty = ob_ty_assets
            .get(ob_ty_handle)
            .expect("Expected object to be loaded.");
        let mut sprite_animation_set = get_animation_set(&mut sprite_acs, *entity)
            .expect("Sprite animation should exist as entity should be valid.");
        let mut body_animation_set = get_animation_set(&mut body_frame_acs, *entity)
            .expect("Body animation should exist as entity should be valid.");
        let mut interaction_animation_set = get_animation_set(&mut interaction_acs, *entity)
            .expect("Interaction animation should exist as entity should be valid.");

        // TODO: replace with actual
        // 1. Update `LastTrackerSystem<T> to take in associated type for what it is tracking from T.
        // 2. Track `SequenceId` from `ObjectStatus`.
        // 3. Add LastTrackerSystem that runs after this one.
        // 4. This system reads `Last<ObjectStatus, Tracked = SeqId>` for this value.
        let last_sequence_id = object_status.sequence_id;
        let next_sequence_id = object_status.sequence_id;

        // TODO:
        // 1. impl Asset for Object<SeqId>
        // 2. attach Handle<Object<SeqId>> to ob_ty (character)
        // 3. read it here, and get the animations
        let animations = &ob_ty
            .object
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

impl<'s, ObTy, SeqId> System<'s> for ObjectAnimationUpdateSystem<ObTy, SeqId>
where
    ObTy: Asset,
    SeqId: SequenceId + 'static,
{
    type SystemData = ObjectAnimationUpdateSystemData<'s, ObTy, SeqId>;

    fn run(
        &mut self,
        (
            entities,
            object_statuses,
            ob_ty_handles,
            ob_ty_assets,
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
            .read(
                self.component_ev_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for ObjectAnimationUpdateSystem."),
            )
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
                &ob_ty_handles,
                object_status_modifications,
            )
                .join()
                .for_each(|(entity, object_status, ob_ty_handle, _)| {
                    Self::swap_animation(
                        &ob_ty_assets,
                        &mut object_animation_storages,
                        entity,
                        object_status,
                        ob_ty_handle,
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
