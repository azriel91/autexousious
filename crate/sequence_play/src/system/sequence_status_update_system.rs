use std::marker::PhantomData;

use amethyst::{
    ecs::{
        storage::{ComponentEvent, Tracked},
        BitSet, Component, Entities, Join, ReadStorage, ReaderId, System, SystemData, WriteStorage,
    },
    shred::Resources,
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{config::SequenceId, play::SequenceStatus};
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Updates `SequenceStatus`s when `SequenceId` changes.
///
/// This **must** run before `SequenceUpdateSystem`, as that relies on the `SequenceStatus` to
/// determine if a `SequenceBegin` event should be sent.
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceStatusUpdateSystem<SeqId>
where
    SeqId: SequenceId,
{
    /// Reader ID for sequence ID changes.
    #[new(default)]
    sequence_id_rid: Option<ReaderId<ComponentEvent>>,
    /// Pre-allocated bitset to track insertions and modifications to `SeqId`s.
    #[new(default)]
    sequence_id_updates: BitSet,
    /// Marker.
    phantom_data: PhantomData<SeqId>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceStatusUpdateSystemData<'s, SeqId>
where
    SeqId: SequenceId,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `SeqId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: ReadStorage<'s, SeqId>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
}

impl<'s, SeqId> System<'s> for SequenceStatusUpdateSystem<SeqId>
where
    SeqId: SequenceId,
    <SeqId as Component>::Storage: Tracked,
{
    type SystemData = SequenceStatusUpdateSystemData<'s, SeqId>;

    fn run(
        &mut self,
        SequenceStatusUpdateSystemData {
            entities,
            sequence_ids,
            mut sequence_statuses,
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

        (&entities, &self.sequence_id_updates)
            .join()
            .for_each(|(entity, _)| {
                sequence_statuses
                    .insert(entity, SequenceStatus::Begin)
                    .expect("Failed to insert `SequenceStatus` component.");
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.sequence_id_rid = Some(WriteStorage::<SeqId>::fetch(&res).register_reader());
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use sequence_model::play::SequenceStatus;
    use test_object_model::config::TestObjectSequenceId;

    use super::SequenceStatusUpdateSystem;

    #[test]
    fn attaches_handle_for_sequence_id_insertions() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, None),
            |world| insert_sequence(world, TestObjectSequenceId::Zero),
            Some(SequenceStatus::Begin),
        )
    }

    #[test]
    fn attaches_handle_for_sequence_id_modifications() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, Some(TestObjectSequenceId::Zero)),
            |world| update_sequence(world, TestObjectSequenceId::One),
            Some(SequenceStatus::Begin),
        )
    }

    fn run_test(
        entity_create_fn: fn(&mut World),
        sequence_id_alter_fn: fn(&mut World),
        sequence_status_expected: Option<SequenceStatus>,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(
                SequenceStatusUpdateSystem::<TestObjectSequenceId>::new(),
                "",
                &[],
            )
            .with_setup(entity_create_fn)
            .with_effect(sequence_id_alter_fn)
            .with_assertion(move |world| expect_sequence_status(world, sequence_status_expected))
            .run_isolated()
    }

    fn insert_sequence(world: &mut World, sequence_id: TestObjectSequenceId) {
        let entity = *world.read_resource::<Entity>();

        let mut sequence_ids = world.write_storage::<TestObjectSequenceId>();
        sequence_ids
            .insert(entity, sequence_id)
            .expect("Failed to insert `TestObjectSequenceId`.");
    }

    fn update_sequence(world: &mut World, sequence_id: TestObjectSequenceId) {
        let entity = *world.read_resource::<Entity>();

        let mut sequence_ids = world.write_storage::<TestObjectSequenceId>();
        let sid = sequence_ids
            .get_mut(entity)
            .expect("Expected entity to contain `TestObjectSequenceId` component.");
        *sid = sequence_id;
    }

    fn create_entity(world: &mut World, sequence_id: Option<TestObjectSequenceId>) {
        let mut entity_builder = world.create_entity();
        if let Some(sequence_id) = sequence_id {
            entity_builder = entity_builder.with(sequence_id);
        }
        let entity = entity_builder.build();

        world.add_resource(entity);
    }

    fn expect_sequence_status(world: &mut World, sequence_status: Option<SequenceStatus>) {
        let entity = *world.read_resource::<Entity>();
        let sequence_statuses = world.read_storage::<SequenceStatus>();
        let sequence_status_actual = sequence_statuses.get(entity).copied();

        assert_eq!(sequence_status, sequence_status_actual);
    }
}
