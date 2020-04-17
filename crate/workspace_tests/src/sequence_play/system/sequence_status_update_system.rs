#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use sequence_model::{
        loaded::SequenceId,
        play::{SequenceStatus, SequenceUpdateEvent},
    };

    use sequence_play::SequenceStatusUpdateSystem;

    #[test]
    fn attaches_handle_and_sends_event_for_sequence_id_insertions() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, None),
            |world| insert_sequence(world, SequenceId::new(0)),
            Some(SequenceStatus::Begin),
            |world| sequence_begin_events(world, SequenceId::new(0)),
        )
    }

    #[test]
    fn attaches_handle_and_sends_event_for_sequence_id_modifications() -> Result<(), Error> {
        run_test(
            |world| create_entity(world, Some(SequenceId::new(0))),
            |world| update_sequence(world, SequenceId::new(1)),
            Some(SequenceStatus::Begin),
            |world| sequence_begin_events(world, SequenceId::new(1)),
        )
    }

    fn run_test(
        entity_create_fn: fn(&mut World),
        sequence_id_alter_fn: fn(&mut World),
        sequence_status_expected: Option<SequenceStatus>,
        sequence_update_events_expected_fn: fn(&mut World) -> Vec<SequenceUpdateEvent>,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(SequenceStatusUpdateSystem::new(), "", &[])
            .with_effect(entity_create_fn)
            .with_effect(register_reader)
            .with_effect(sequence_id_alter_fn)
            .with_assertion(move |world| expect_sequence_status(world, sequence_status_expected))
            .with_assertion(move |world| {
                let events_expected = sequence_update_events_expected_fn(world);
                expect_events(world, events_expected);
            })
            .run_winit_loop()
    }

    fn register_reader(world: &mut World) {
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
            ec.register_reader()
        }; // kcov-ignore
        world.insert(reader_id);
    }

    fn insert_sequence(world: &mut World, sequence_id: SequenceId) {
        let entity = *world.read_resource::<Entity>();

        let mut sequence_ids = world.write_storage::<SequenceId>();
        sequence_ids
            .insert(entity, sequence_id)
            .expect("Failed to insert `SequenceId`.");
    }

    fn update_sequence(world: &mut World, sequence_id: SequenceId) {
        let entity = *world.read_resource::<Entity>();

        let mut sequence_ids = world.write_storage::<SequenceId>();
        let sid = sequence_ids
            .get_mut(entity)
            .expect("Expected entity to contain `SequenceId` component.");
        *sid = sequence_id;
    }

    fn create_entity(world: &mut World, sequence_id: Option<SequenceId>) {
        let mut entity_builder = world.create_entity();
        if let Some(sequence_id) = sequence_id {
            entity_builder = entity_builder.with(sequence_id);
        }
        let entity = entity_builder.build();

        world.insert(entity);
    }

    fn sequence_begin_events(
        world: &mut World,
        sequence_id: SequenceId,
    ) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id,
        }]
    }

    fn expect_events(world: &mut World, events_expected: Vec<SequenceUpdateEvent>) {
        let target_entity = *world.read_resource::<Entity>();
        let mut reader_id = world.write_resource::<ReaderId<SequenceUpdateEvent>>();
        let ec = world.read_resource::<EventChannel<SequenceUpdateEvent>>();

        // Map owned values into references.
        let events_expected = events_expected.iter().collect::<Vec<_>>();

        // Filter events for the entity we care about.
        let events_actual = ec
            .read(&mut reader_id)
            .filter(|ev| match ev {
                SequenceUpdateEvent::SequenceBegin { entity, .. }
                | SequenceUpdateEvent::FrameBegin { entity, .. }
                | SequenceUpdateEvent::SequenceEnd { entity, .. } => target_entity == *entity,
            })
            .collect::<Vec<_>>();

        assert_eq!(events_expected, events_actual)
    }

    fn expect_sequence_status(world: &mut World, sequence_status: Option<SequenceStatus>) {
        let entity = *world.read_resource::<Entity>();
        let sequence_statuses = world.read_storage::<SequenceStatus>();
        let sequence_status_actual = sequence_statuses.get(entity).copied();

        assert_eq!(sequence_status, sequence_status_actual);
    }
}
