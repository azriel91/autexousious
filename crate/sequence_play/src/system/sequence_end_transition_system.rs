use amethyst::{
    ecs::{Entities, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    loaded::{SequenceEndTransition, SequenceId},
    play::SequenceUpdateEvent,
};
use typename_derive::TypeName;

/// Transitions an object when their `SequenceUpdateEvent::SequenceEnd`
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceEndTransitionSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    sequence_update_event_rid: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceEndTransitionSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `SequenceEndTransition` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: ReadStorage<'s, SequenceEndTransition>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
}

impl<'s> System<'s> for SequenceEndTransitionSystem {
    type SystemData = SequenceEndTransitionSystemData<'s>;

    fn run(
        &mut self,
        SequenceEndTransitionSystemData {
            entities,
            sequence_update_ec,
            sequence_end_transitions,
            mut sequence_ids,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(self.sequence_update_event_rid.as_mut().expect(
                "Expected `sequence_update_event_rid` to exist for \
                 `SequenceEndTransitionSystem`.",
            ))
            .filter(|ev| {
                if let SequenceUpdateEvent::SequenceEnd { .. } = ev {
                    true
                } else {
                    false
                }
            })
            .for_each(|ev| {
                let entity = ev.entity();

                let sequence_end_transition = sequence_end_transitions.get(entity).copied();

                if let Some(sequence_end_transition) = sequence_end_transition {
                    match sequence_end_transition {
                        SequenceEndTransition::None => {}
                        SequenceEndTransition::Repeat => {
                            let sequence_id = sequence_ids
                                .get(entity)
                                .copied()
                                .expect("Expected entity to have `SequenceId` component.");
                            // Re-insertion causes sequence to restart.
                            sequence_ids
                                .insert(entity, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
                        SequenceEndTransition::Delete => {
                            entities
                                .delete(entity)
                                .expect("Failed to delete entity on `SequenceEndTransition`.");
                        }
                        SequenceEndTransition::SequenceId(sequence_id) => {
                            sequence_ids
                                .insert(entity, sequence_id)
                                .expect("Failed to insert `SequenceId` component.");
                        }
                    }
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sequence_update_event_rid = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

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

    use super::SequenceEndTransitionSystem;

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
            .with_setup(move |world| {
                // Add entity before registering `component_event_rid`, so we don't get the first
                // insertion event.
                let entity = world
                    .create_entity()
                    .with(sequence_id_setup)
                    .with(sequence_end_transition)
                    .build();
                world.insert(entity);
            })
            .with_setup(register_sequence_id_reader)
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
        SequenceUpdateEvent::SequenceBegin { entity }
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
