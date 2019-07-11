use std::marker::PhantomData;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{
        storage::{ComponentEvent, Tracked},
        BitSet, Component, Entities, Join, Read, ReadStorage, ReaderId, System, SystemData,
        WriteStorage,
    },
    shred::Resources,
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use object_model::loaded::{GameObject, ObjectWrapper};
use object_prefab::FrameComponentDataHandleStorages;
use sequence_model::config::SequenceEndTransition;
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Updates the attached `Handle<FrameComponentData>`s when `O::SequenceId` changes.
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceComponentUpdateSystem<O>
where
    O: GameObject,
{
    /// Reader ID for sequence ID changes.
    #[new(default)]
    sequence_id_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to `O::SequenceId`s.
    #[new(default)]
    sequence_id_updates: BitSet,
    /// Marker.
    phantom_data: PhantomData<O>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentUpdateSystemData<'s, O>
where
    O: GameObject,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `O::SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, O::SequenceId>,
    /// `Handle<O::ObjectWrapper>` components.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    /// `O::ObjectWrapper` assets.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `SequenceEndTransition<O::SequenceId>` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: WriteStorage<'s, SequenceEndTransition<O::SequenceId>>,
    /// Frame component data handle storages.
    #[derivative(Debug = "ignore")]
    pub frame_component_data_handle_storages: FrameComponentDataHandleStorages<'s>,
}

impl<'s, O> System<'s> for SequenceComponentUpdateSystem<O>
where
    O: GameObject,
    <O::SequenceId as Component>::Storage: Tracked,
{
    type SystemData = SequenceComponentUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        SequenceComponentUpdateSystemData {
            entities,
            sequence_ids,
            object_wrapper_handles,
            object_wrapper_assets,
            mut sequence_end_transitions,
            frame_component_data_handle_storages:
                FrameComponentDataHandleStorages {
                    mut wait_sequence_handles,
                    mut sprite_render_sequence_handles,
                    mut body_sequence_handles,
                    mut interactions_sequence_handles,
                    mut spawns_sequence_handles,
                },
        }: Self::SystemData,
    ) {
        self.sequence_id_updates.clear();

        sequence_ids
            .channel()
            .read(
                self.sequence_id_rid
                    .as_mut()
                    .expect("Expected `sequence_id_rid` to be set."),
            )
            .for_each(|event| match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.sequence_id_updates.add(*id);
                }
                ComponentEvent::Removed(_id) => {}
            });

        (
            &entities,
            &sequence_ids,
            &object_wrapper_handles,
            &self.sequence_id_updates,
        )
            .join()
            .for_each(|(entity, sequence_id, object_wrapper_handle, _)| {
                let components = {
                    let object_wrapper = object_wrapper_assets
                        .get(&object_wrapper_handle)
                        .expect("Expected `ObjectWrapper` to be loaded.");
                    let object = object_wrapper.inner();

                    (
                        object.sequence_end_transitions.get(&sequence_id).copied(),
                        object.wait_sequence_handles.get(&sequence_id),
                        object.sprite_render_sequence_handles.get(&sequence_id),
                        object.body_sequence_handles.get(&sequence_id),
                        object.interactions_sequence_handles.get(&sequence_id),
                        object.spawns_sequence_handles.get(&sequence_id),
                    )
                };

                if let (
                    Some(sequence_end_transition),
                    Some(wait_sequence_handle),
                    Some(sprite_render_sequence_handle),
                    Some(body_sequence_handle),
                    Some(interactions_sequence_handle),
                    Some(spawns_sequence_handle),
                ) = components
                {
                    sequence_end_transitions
                        .insert(entity, sequence_end_transition)
                        .expect("Failed to insert `SequenceEndTransition` component.");
                    wait_sequence_handles
                        .insert(entity, wait_sequence_handle.clone())
                        .expect("Failed to insert `WaitSequenceHandle` component.");
                    sprite_render_sequence_handles
                        .insert(entity, sprite_render_sequence_handle.clone())
                        .expect("Failed to insert `SpriteRenderSequenceHandle` component.");
                    body_sequence_handles
                        .insert(entity, body_sequence_handle.clone())
                        .expect("Failed to insert `BodySequenceHandle` component.");
                    interactions_sequence_handles
                        .insert(entity, interactions_sequence_handle.clone())
                        .expect("Failed to insert `InteractionsSequenceHandle` component.");
                    spawns_sequence_handles
                        .insert(entity, spawns_sequence_handle.clone())
                        .expect("Failed to insert `SpawnsSequenceHandle` component.");
                } else {
                    error!(
                        "Expected all frame component data handles to exist for sequence ID: \
                         `{:?}`, but was {:?}.",
                        sequence_id, &components
                    );
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.sequence_id_rid = Some(WriteStorage::<O::SequenceId>::fetch(&res).register_reader());
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World},
        Error,
    };
    use application_test_support::{AutexousiousApplication, ObjectQueries, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use character_model::{config::CharacterSequenceId, loaded::Character};
    use collision_model::loaded::{BodySequenceHandle, InteractionsSequenceHandle};
    use sequence_model::{config::SequenceEndTransition, loaded::WaitSequenceHandle};
    use spawn_model::loaded::SpawnsSequenceHandle;
    use sprite_model::loaded::SpriteRenderSequenceHandle;

    use super::SequenceComponentUpdateSystem;

    #[test]
    fn attaches_handle_for_sequence_id_insertions() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(SequenceComponentUpdateSystem::<Character>::new(), "", &[])
            .with_setup(|world| insert_sequence(world, CharacterSequenceId::RunStop))
            .with_assertion(|world| expect_components(world, CharacterSequenceId::RunStop))
            .run_isolated()
    }

    #[test]
    fn attaches_handle_for_sequence_id_modifications() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(SequenceComponentUpdateSystem::<Character>::new(), "", &[])
            .with_setup(|world| update_sequence(world, CharacterSequenceId::RunStop))
            .with_assertion(|world| expect_components(world, CharacterSequenceId::RunStop))
            .run_isolated()
    }

    fn insert_sequence(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = create_entity(world);

        {
            let mut sequence_ids = world.write_storage::<CharacterSequenceId>();
            sequence_ids
                .insert(entity, sequence_id)
                .expect("Failed to insert `CharacterSequenceId`.");
        }

        world.add_resource(entity);
    }

    fn update_sequence(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = create_entity(world);

        {
            let mut sequence_ids = world.write_storage::<CharacterSequenceId>();
            let sid = sequence_ids
                .get_mut(entity)
                .expect("Expected entity to contain `CharacterSequenceId` component.");
            *sid = sequence_id;
        }

        world.add_resource(entity);
    }

    fn create_entity(world: &mut World) -> Entity {
        let asset_slug = CHAR_BAT_SLUG.clone();
        let object_wrapper_handle = ObjectQueries::object_wrapper_handle(world, &asset_slug);
        let wait_sequence_handle =
            SequenceQueries::wait_sequence_handle(world, &asset_slug, CharacterSequenceId::Stand);

        world
            .create_entity()
            .with(object_wrapper_handle)
            .with(CharacterSequenceId::Stand)
            .with(wait_sequence_handle)
            .build()
    }

    fn expect_components(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = *world.read_resource::<Entity>();

        macro_rules! assert_component_attached {
            ($component_kind:ident, $component_type:path) => {
                let expected_handle =
                    SequenceQueries::$component_kind(world, &CHAR_BAT_SLUG.clone(), sequence_id);
                let components = world.read_storage::<$component_type>();
                assert_eq!(
                    &expected_handle,
                    components.get(entity).expect(concat!(
                        "Expected entity to contain `",
                        stringify!($component_type),
                        "` component."
                    ))
                );
            };
        }

        assert_component_attached!(
            sequence_end_transition,
            SequenceEndTransition<CharacterSequenceId>
        );
        assert_component_attached!(wait_sequence_handle, WaitSequenceHandle);
        assert_component_attached!(sprite_render_sequence_handle, SpriteRenderSequenceHandle);
        assert_component_attached!(body_sequence_handle, BodySequenceHandle);
        assert_component_attached!(interactions_sequence_handle, InteractionsSequenceHandle);
        assert_component_attached!(spawns_sequence_handle, SpawnsSequenceHandle);
    }
}
