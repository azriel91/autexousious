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
use named_type::NamedType;
use named_type_derive::NamedType;
use object_model::loaded::{GameObject, ObjectWrapper};
use object_prefab::ComponentSequenceHandleStorages;
use sequence_model::loaded::ComponentSequencesHandle;
use shred_derive::SystemData;

/// Updates the attached `ComponentSequencesHandle`s when the `SequenceId` is changed.
#[derive(Debug, Default, NamedType, new)]
pub struct ComponentSequencesUpdateSystem<O>
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
pub struct ComponentSequencesUpdateSystemData<'s, O>
where
    O: GameObject,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, O::SequenceId>,
    /// `Handle<O::ObjectWrapper>` components.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    /// `O::ObjectWrapper` assets.
    #[derivative(Debug = "ignore")]
    pub object_wrapper_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `ComponentSequencesHandle` components.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: WriteStorage<'s, ComponentSequencesHandle>,
    /// Component sequence handle storages.
    #[derivative(Debug = "ignore")]
    pub component_sequence_handle_storages: ComponentSequenceHandleStorages<'s>,
}

impl<'s, O> System<'s> for ComponentSequencesUpdateSystem<O>
where
    O: GameObject,
    <O::SequenceId as Component>::Storage: Tracked,
{
    type SystemData = ComponentSequencesUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        ComponentSequencesUpdateSystemData {
            entities,
            sequence_ids,
            object_wrapper_handles,
            object_wrapper_assets,
            mut component_sequences_handles,
            component_sequence_handle_storages:
                ComponentSequenceHandleStorages {
                    mut wait_sequence_handles,
                    mut sprite_render_sequence_handles,
                    mut body_sequence_handles,
                    mut interactions_sequence_handles,
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
                let component_sequence_handleses = {
                    let object_wrapper = object_wrapper_assets
                        .get(&object_wrapper_handle)
                        .expect("Expected `ObjectWrapper` to be loaded.");
                    let object = object_wrapper.inner();

                    (
                        object.component_sequences_handles.get(&sequence_id),
                        object.wait_sequence_handles.get(&sequence_id),
                        object.sprite_render_sequence_handles.get(&sequence_id),
                        object.body_sequence_handles.get(&sequence_id),
                        object.interactions_sequence_handles.get(&sequence_id),
                    )
                };

                if let (
                    Some(component_sequences_handle),
                    Some(wait_sequence_handle),
                    Some(sprite_render_sequence_handle),
                    Some(body_sequence_handle),
                    Some(interactions_sequence_handle),
                ) = component_sequence_handleses
                {
                    component_sequences_handles
                        .insert(entity, component_sequences_handle.clone())
                        .expect("Failed to insert `ComponentSequencesHandle` component.");
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
                } else {
                    error!(
                        "Expected all component sequence handles to exist for sequence ID: `{:?}`, \
                         but was {:?}.",
                        sequence_id, &component_sequence_handleses
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
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_model::{config::CharacterSequenceId, loaded::Character};
    use sequence_model::loaded::ComponentSequencesHandle;

    use super::ComponentSequencesUpdateSystem;

    #[test]
    fn attaches_handle_for_sequence_id_insertions() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(ComponentSequencesUpdateSystem::<Character>::new(), "", &[])
            .with_setup(|world| insert_sequence(world, CharacterSequenceId::RunStop))
            .with_assertion(|world| expect_component_sequences(world, CharacterSequenceId::RunStop))
            .run_isolated()
    }

    #[test]
    fn attaches_handle_for_sequence_id_modifications() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(ComponentSequencesUpdateSystem::<Character>::new(), "", &[])
            .with_setup(|world| update_sequence(world, CharacterSequenceId::RunStop))
            .with_assertion(|world| expect_component_sequences(world, CharacterSequenceId::RunStop))
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
        let asset_slug = ASSETS_CHAR_BAT_SLUG.clone();
        let object_wrapper_handle = ObjectQueries::object_wrapper_handle(world, &asset_slug);
        let component_sequences_handle = SequenceQueries::component_sequences_handle(
            world,
            &asset_slug,
            CharacterSequenceId::Stand,
        );

        world
            .create_entity()
            .with(object_wrapper_handle)
            .with(CharacterSequenceId::Stand)
            .with(component_sequences_handle)
            .build()
    }

    fn expect_component_sequences(world: &mut World, sequence_id: CharacterSequenceId) {
        let entity = *world.read_resource::<Entity>();
        let expected_handle = SequenceQueries::component_sequences_handle(
            world,
            &ASSETS_CHAR_BAT_SLUG.clone(),
            sequence_id,
        );
        let component_sequences_handles = world.read_storage::<ComponentSequencesHandle>();

        assert_eq!(
            &expected_handle,
            component_sequences_handles
                .get(entity)
                .expect("Expected entity to contain `ComponentSequencesHandle` component.")
        );
    }
}
