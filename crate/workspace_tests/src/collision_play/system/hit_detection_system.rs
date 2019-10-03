#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind},
        play::{
            ContactEvent, HitEvent, HitObjectCount, HitRepeatClock, HitRepeatTracker,
            HitRepeatTrackers,
        },
    };
    use pretty_assertions::assert_eq;
    use shape_model::Volume;
    use slotmap::SlotMap;

    use collision_play::HitDetectionSystem;

    const HIT_LIMIT: u32 = 3;

    #[test]
    fn inserts_hit_event_when_hit_repeat_trackers_does_not_exist() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_effect(setup_event_reader)
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                send_event(world, contact_event(entity_from, entity_to));

                world.insert((entity_from, entity_to));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to) = *world.read_resource::<(Entity, Entity)>();
                assert_events(world, vec![hit_event(entity_from, entity_to)]);
            })
            .run()
    }

    #[test]
    fn inserts_hit_event_when_hit_repeat_trackers_does_not_contain_entity_to() -> Result<(), Error>
    {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_effect(setup_event_reader)
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();
                let entity_other = world.create_entity().build();

                {
                    let mut hit_repeat_trackerses = world.write_storage::<HitRepeatTrackers>();
                    hit_repeat_trackerses
                        .insert(entity_from, hit_repeat_trackers(entity_other))
                        .expect("Failed to insert `HitRepeatTrackers`.");
                }
                send_event(world, contact_event(entity_from, entity_to));

                world.insert((entity_from, entity_to));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to) = *world.read_resource::<(Entity, Entity)>();
                assert_events(world, vec![hit_event(entity_from, entity_to)]);
            })
            .run()
    }

    #[test]
    fn does_not_insert_hit_event_when_hit_repeat_trackers_contains_entity_to() -> Result<(), Error>
    {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_effect(setup_event_reader)
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                {
                    let mut hit_repeat_trackerses = world.write_storage::<HitRepeatTrackers>();
                    hit_repeat_trackerses
                        .insert(entity_from, hit_repeat_trackers(entity_to))
                        .expect("Failed to insert `HitRepeatTrackers`.");
                }
                send_event(world, contact_event(entity_from, entity_to));

                world.insert((entity_from, entity_to));
            })
            .with_assertion(|world| {
                assert_events(world, vec![]);
            })
            .run()
    }

    #[test]
    fn limits_hit_events_to_remaining_hit_limit_count() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_effect(setup_event_reader)
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_tos = (0..HIT_LIMIT)
                    .map(|_| {
                        let entity_to = world.create_entity().build();
                        send_event(world, contact_event(entity_from, entity_to));
                        entity_to
                    })
                    .collect::<Vec<Entity>>();

                // Send extra contact events, these should not map to hit events.
                let contact_events = (0..5)
                    .map(|_| {
                        let entity_extra = world.create_entity().build();
                        contact_event(entity_from, entity_extra)
                    })
                    .collect::<Vec<ContactEvent>>();
                send_events(world, contact_events);

                world.insert((entity_from, entity_tos));
            })
            .with_assertion(|world| {
                let (entity_from, entity_tos) = {
                    let (entity_from, ref mut entity_tos) =
                        &mut *world.write_resource::<(Entity, Vec<Entity>)>();
                    (*entity_from, entity_tos.drain(..).collect::<Vec<Entity>>())
                };
                let hit_events = entity_tos
                    .iter()
                    .map(|entity_to| hit_event(entity_from, *entity_to))
                    .collect::<Vec<HitEvent>>();
                assert_events(world, hit_events);

                assert_eq!(0, world.read_storage::<HitObjectCount>().count());
            })
            .run()
    }

    #[test]
    fn does_not_limit_hit_events_to_when_hit_limit_unlimited() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_effect(setup_event_reader)
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_tos = (0..(HIT_LIMIT + 5))
                    .map(|_| {
                        let entity_to = world.create_entity().build();
                        send_event(
                            world,
                            contact_event_with_hit_limit(
                                entity_from,
                                entity_to,
                                HitLimit::Unlimited,
                            ),
                        );
                        entity_to
                    })
                    .collect::<Vec<Entity>>();

                world.insert((entity_from, entity_tos));
            })
            .with_assertion(|world| {
                let (entity_from, entity_tos) = {
                    let (entity_from, ref mut entity_tos) =
                        &mut *world.write_resource::<(Entity, Vec<Entity>)>();
                    (*entity_from, entity_tos.drain(..).collect::<Vec<Entity>>())
                };
                let hit_events = entity_tos
                    .iter()
                    .map(|entity_to| {
                        hit_event_with_hit_limit(entity_from, *entity_to, HitLimit::Unlimited)
                    })
                    .collect::<Vec<HitEvent>>();
                assert_events(world, hit_events);
            })
            .run()
    }

    fn setup_event_reader(world: &mut World) {
        let hit_event_rid = world
            .write_resource::<EventChannel<HitEvent>>()
            .register_reader(); // kcov-ignore

        world.insert(hit_event_rid);
    }

    fn send_event(world: &mut World, event: ContactEvent) {
        let mut ec = world.write_resource::<EventChannel<ContactEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    fn send_events(world: &mut World, events: Vec<ContactEvent>) {
        let mut ec = world.write_resource::<EventChannel<ContactEvent>>();
        ec.iter_write(events)
    } // kcov-ignore

    fn hit_repeat_trackers(entity_to: Entity) -> HitRepeatTrackers {
        let mut slot_map = SlotMap::new();
        slot_map.insert(HitRepeatTracker::new(entity_to, HitRepeatClock::new(4)));
        HitRepeatTrackers::new(slot_map)
    }

    fn contact_event(entity_from: Entity, entity_to: Entity) -> ContactEvent {
        contact_event_with_hit_limit(entity_from, entity_to, HitLimit::Limit(HIT_LIMIT))
    }

    fn contact_event_with_hit_limit(
        entity_from: Entity,
        entity_to: Entity,
        hit_limit: HitLimit,
    ) -> ContactEvent {
        ContactEvent::new(entity_from, entity_to, interaction(hit_limit), body())
    }

    fn hit_event(entity_from: Entity, entity_to: Entity) -> HitEvent {
        hit_event_with_hit_limit(entity_from, entity_to, HitLimit::Limit(HIT_LIMIT))
    }

    fn hit_event_with_hit_limit(
        entity_from: Entity,
        entity_to: Entity,
        hit_limit: HitLimit,
    ) -> HitEvent {
        HitEvent::new(entity_from, entity_to, interaction(hit_limit), body())
    }

    fn interaction(hit_limit: HitLimit) -> Interaction {
        Interaction::new(
            InteractionKind::Hit(Hit {
                repeat_delay: HitRepeatDelay::new(4),
                hit_limit,
                ..Default::default()
            }),
            vec![],
            true,
        )
    }

    fn body() -> Volume {
        Volume::Box {
            x: 0,
            y: 0,
            z: 0,
            w: 1,
            h: 1,
            d: 1,
        }
    }

    fn assert_events(world: &mut World, hit_events_expected: Vec<HitEvent>) {
        let hit_ec = world.read_resource::<EventChannel<HitEvent>>();
        let mut hit_event_rid = world.write_resource::<ReaderId<HitEvent>>();
        let hit_events = hit_ec.read(&mut hit_event_rid).collect::<Vec<&HitEvent>>();

        let hit_events_expected = hit_events_expected.iter().collect::<Vec<&HitEvent>>();
        assert_eq!(hit_events_expected, hit_events);
    }
}
