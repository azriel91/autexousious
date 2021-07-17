#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{
            storage::ComponentEvent, world::EntitiesRes, Builder, Entity, ReadStorage, World,
            WorldExt, WriteExpect,
        },
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use sequence_model::{
        loaded::{SequenceEndTransition, SequenceId},
        play::SequenceUpdateEvent,
    };

    use sequence_play::SequenceEndTransitionSystem;

    #[test]
    fn does_nothing_on_transition_none() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                sequence_id: SequenceId::new(1),
                sequence_end_transition: SequenceEndTransition::None,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: SequenceId::new(1),
                alive_expected: true,
                event_expected_fn: |events_actual, _entity| {
                    assert!(events_actual.is_empty());
                },
            },
        )
    }

    #[test]
    fn does_nothing_on_sequence_begin_event() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                sequence_id: SequenceId::new(1),
                sequence_end_transition: SequenceEndTransition::Repeat,
                sequence_update_event_fn: Some(sequence_begin_event),
            },
            ParamsExpected {
                sequence_id: SequenceId::new(1),
                alive_expected: true,
                event_expected_fn: |events_actual, _entity| {
                    assert!(events_actual.is_empty());
                },
            },
        )
    }

    #[test]
    fn inserts_same_sequence_id_on_transition_repeat() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                sequence_id: SequenceId::new(1),
                sequence_end_transition: SequenceEndTransition::Repeat,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: SequenceId::new(1),
                alive_expected: true,
                event_expected_fn: |events_actual, entity| {
                    assert_eq!(1, events_actual.len());
                    if let ComponentEvent::Inserted(index) | ComponentEvent::Modified(index) =
                        &events_actual[0]
                    {
                        assert_eq!(entity.id(), *index);
                    } else {
                        // kcov-ignore-start
                        panic!(
                            "Expected `ComponentEvent::Inserted` or `ComponentEvent::Modified` \
                             event."
                        );
                        // kcov-ignore-end
                    }
                },
            },
        )
    }

    #[test]
    fn inserts_next_sequence_id_on_transition_sequence_id() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                sequence_id: SequenceId::new(1),
                sequence_end_transition: SequenceEndTransition::SequenceId(SequenceId::new(0)),
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: SequenceId::new(0),
                alive_expected: true,
                event_expected_fn: |events_actual, entity| {
                    assert_eq!(1, events_actual.len());
                    if let ComponentEvent::Inserted(index) | ComponentEvent::Modified(index) =
                        &events_actual[0]
                    {
                        assert_eq!(entity.id(), *index);
                    } else {
                        // kcov-ignore-start
                        panic!(
                            "Expected `ComponentEvent::Inserted` or `ComponentEvent::Modified` \
                             event."
                        );
                        // kcov-ignore-end
                    }
                },
            },
        )
    }

    #[test]
    fn deletes_on_transition_delete() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                sequence_id: SequenceId::new(1),
                sequence_end_transition: SequenceEndTransition::Delete,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: SequenceId::new(0),
                alive_expected: false,
                event_expected_fn: |_, _| {},
            },
        )
    }

    fn run_test(
        ParamsSetup {
            sequence_id: sequence_id_setup,
            sequence_end_transition,
            sequence_update_event_fn,
        }: ParamsSetup,
        ParamsExpected {
            sequence_id: sequence_id_expected,
            alive_expected,
            event_expected_fn,
        }: ParamsExpected,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(SequenceEndTransitionSystem::new(), "", &[])
            .with_effect(move |world| {
                // Add entity before registering `component_event_rid`, so we don't get the
                // first insertion event.
                let entity = world
                    .create_entity()
                    .with(sequence_id_setup)
                    .with(sequence_end_transition)
                    .build();
                world.insert(entity);
            })
            .with_effect(register_sequence_id_reader)
            .with_effect(move |world| {
                if let Some(sequence_update_event_fn) = sequence_update_event_fn {
                    let entity = *world.read_resource::<Entity>();
                    let sequence_update_event = sequence_update_event_fn(entity);
                    send_event(world, sequence_update_event)
                }
            })
            .with_assertion(move |world| {
                world.maintain();
                if alive_expected {
                    let events_actual = {
                        let (test_object_sequence_names, mut component_event_rid) = world
                            .system_data::<(
                                ReadStorage<'_, SequenceId>,
                                WriteExpect<'_, ReaderId<ComponentEvent>>,
                            )>();

                        test_object_sequence_names
                            .channel()
                            .read(&mut component_event_rid)
                            .map(Clone::clone)
                            .collect::<Vec<ComponentEvent>>()
                    };
                    let entities = world.read_resource::<EntitiesRes>();
                    let entity = *world.read_resource::<Entity>();
                    let sequence_id_actual = {
                        let test_object_sequence_names = world.read_storage::<SequenceId>();
                        test_object_sequence_names
                            .get(entity)
                            .copied()
                            .expect("Expected entity to have `SequenceId` component.")
                    };
                    assert!(entities.is_alive(entity));
                    assert_eq!(sequence_id_expected, sequence_id_actual);

                    event_expected_fn(events_actual, entity);
                } else {
                    let entities = world.read_resource::<EntitiesRes>();
                    let entity = *world.read_resource::<Entity>();

                    assert_eq!(false, entities.is_alive(entity));
                }
            })
            .run()
    }

    fn send_event(world: &mut World, event: SequenceUpdateEvent) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.single_write(event)
    }

    fn sequence_begin_event(entity: Entity) -> SequenceUpdateEvent {
        SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SequenceId::new(1),
        }
    }

    fn sequence_end_event(entity: Entity) -> SequenceUpdateEvent {
        SequenceUpdateEvent::SequenceEnd {
            entity,
            frame_index: 0,
        }
    }

    fn register_sequence_id_reader(world: &mut World) {
        let component_event_rid = {
            let mut test_object_sequence_names = world.write_storage::<SequenceId>();
            test_object_sequence_names.register_reader()
        };
        world.insert(component_event_rid);
    }

    struct ParamsSetup {
        sequence_id: SequenceId,
        sequence_end_transition: SequenceEndTransition,
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
    }

    struct ParamsExpected {
        sequence_id: SequenceId,
        alive_expected: bool,
        event_expected_fn: fn(Vec<ComponentEvent>, Entity),
    }
}
