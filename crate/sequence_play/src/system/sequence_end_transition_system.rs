use std::marker::PhantomData;

use amethyst::{
    ecs::{Entities, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    config::{SequenceEndTransition, SequenceId},
    play::SequenceUpdateEvent,
};
use typename_derive::TypeName;

/// Transitions an object when their `SequenceUpdateEvent::SequenceEnd`
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceEndTransitionSystem<SeqId>
where
    SeqId: SequenceId,
{
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
    /// Marker.
    phantom_data: PhantomData<SeqId>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceEndTransitionSystemData<'s, SeqId>
where
    SeqId: SequenceId,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `SequenceEndTransition<SeqId>` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: ReadStorage<'s, SequenceEndTransition<SeqId>>,
    /// `SeqId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SeqId>,
}

impl<'s, SeqId> System<'s> for SequenceEndTransitionSystem<SeqId>
where
    SeqId: SequenceId,
{
    type SystemData = SequenceEndTransitionSystemData<'s, SeqId>;

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
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for SequenceEndTransitionSystem."),
            )
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
                                .expect("Expected entity to have `SeqId` component.");
                            // Re-insertion causes sequence to restart.
                            sequence_ids
                                .insert(entity, sequence_id)
                                .expect("Failed to insert `SeqId` component.");
                        }
                        SequenceEndTransition::Delete => {
                            entities
                                .delete(entity)
                                .expect("Failed to delete entity on `SequenceEndTransition`.");
                        }
                        SequenceEndTransition::SequenceId(sequence_id) => {
                            sequence_ids
                                .insert(entity, sequence_id)
                                .expect("Failed to insert `SeqId` component.");
                        }
                    }
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
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
    use sequence_model::{config::SequenceEndTransition, play::SequenceUpdateEvent};
    use test_object_model::config::TestObjectSequenceId;

    use super::SequenceEndTransitionSystem;

    #[test]
    fn does_nothing_on_transition_none() -> Result<(), Error> {
        run_test(
            ParamsSetup {
                sequence_id: TestObjectSequenceId::One,
                sequence_end_transition: SequenceEndTransition::None,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: TestObjectSequenceId::One,
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
                sequence_id: TestObjectSequenceId::One,
                sequence_end_transition: SequenceEndTransition::Repeat,
                sequence_update_event_fn: Some(sequence_begin_event),
            },
            ParamsExpected {
                sequence_id: TestObjectSequenceId::One,
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
                sequence_id: TestObjectSequenceId::One,
                sequence_end_transition: SequenceEndTransition::Repeat,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: TestObjectSequenceId::One,
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
                sequence_id: TestObjectSequenceId::One,
                sequence_end_transition: SequenceEndTransition::SequenceId(
                    TestObjectSequenceId::Zero,
                ),
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: TestObjectSequenceId::Zero,
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
                sequence_id: TestObjectSequenceId::One,
                sequence_end_transition: SequenceEndTransition::Delete,
                sequence_update_event_fn: Some(sequence_end_event),
            },
            ParamsExpected {
                sequence_id: TestObjectSequenceId::Zero,
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
            .with_system(
                SequenceEndTransitionSystem::<TestObjectSequenceId>::new(),
                "",
                &[],
            )
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
                        let (test_object_sequence_ids, mut component_event_rid) = world
                            .system_data::<(
                                ReadStorage<'_, TestObjectSequenceId>,
                                WriteExpect<'_, ReaderId<ComponentEvent>>,
                            )>();

                        test_object_sequence_ids
                            .channel()
                            .read(&mut component_event_rid)
                            .map(Clone::clone)
                            .collect::<Vec<ComponentEvent>>()
                    };
                    let entities = world.read_resource::<EntitiesRes>();
                    let entity = *world.read_resource::<Entity>();
                    let sequence_id_actual = {
                        let test_object_sequence_ids = world.read_storage::<TestObjectSequenceId>();
                        test_object_sequence_ids
                            .get(entity)
                            .copied()
                            .expect("Expected entity to have `TestObjectSequenceId` component.")
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
            let mut test_object_sequence_ids = world.write_storage::<TestObjectSequenceId>();
            test_object_sequence_ids.register_reader()
        };
        world.insert(component_event_rid);
    }

    struct ParamsSetup {
        sequence_id: TestObjectSequenceId,
        sequence_end_transition: SequenceEndTransition<TestObjectSequenceId>,
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
    }

    struct ParamsExpected {
        sequence_id: TestObjectSequenceId,
        alive_expected: bool,
        event_expected_fn: fn(Vec<ComponentEvent>, Entity),
    }
}
