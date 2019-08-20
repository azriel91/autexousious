use amethyst::{
    ecs::{Entity, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{
    config::{Hit, HitLimit, Interaction, InteractionKind},
    play::{ContactEvent, HitEvent, HitObjectCount, HitRepeatTrackers},
};
use derive_new::new;
use typename_derive::TypeName;

/// Detects whether a `HitEvent` occurs there is contact between an `Interaction` and a `Volume`.
#[derive(Debug, Default, TypeName, new)]
pub struct HitDetectionSystem {
    /// Reader ID for the `ContactEvent` event channel.
    #[new(default)]
    contact_event_rid: Option<ReaderId<ContactEvent>>,
}

type HitDetectionSystemData<'s> = (
    Read<'s, EventChannel<ContactEvent>>,
    ReadStorage<'s, HitRepeatTrackers>,
    WriteStorage<'s, HitObjectCount>,
    Write<'s, EventChannel<HitEvent>>,
);

impl HitDetectionSystem {
    fn update_hit_object_count(
        hit_object_counts: &mut WriteStorage<'_, HitObjectCount>,
        entity_hitter: Entity,
    ) -> HitObjectCount {
        if let Some(hit_object_count) = hit_object_counts.get_mut(entity_hitter) {
            *hit_object_count += 1;
            *hit_object_count
        } else {
            let hit_object_count = HitObjectCount::new(1);
            hit_object_counts
                .insert(entity_hitter, hit_object_count)
                .expect("Failed to insert `HitObjectCount` component.");
            hit_object_count
        }
    }
}

impl<'s> System<'s> for HitDetectionSystem {
    type SystemData = HitDetectionSystemData<'s>;

    fn run(
        &mut self,
        (contact_ec, hit_repeat_trackerses, mut hit_object_counts, mut hit_ec): Self::SystemData,
    ) {
        let hit_events = contact_ec
            .read(
                self.contact_event_rid
                    .as_mut()
                    .expect("Expected `contact_event_rid` to exist for `HitDetectionSystem`."),
            )
            .filter(|ev| {
                // This assumes `ev.from` is the hitting object entity. If we have a separate entity
                // for each `Interaction`, then this assumption breaks, and we need to traverse the
                // entity hierarchy to find the object entity.
                let entity_hitter = ev.from;

                // This assumes `ev.to` is the hit object entity. If we have a separate
                // entity for each `Body`, then this assumption breaks, and we need to
                // traverse the entity hierarchy to find the object entity.
                let entity_hit = ev.to;

                // Check
                //
                // 1. `HitRepeatTrackers`:
                //
                //     Make sure the same object entity is not hit before the `HitRepeatClock` is up.
                //
                // 2. `HitLimit`: Make sure not more than `HitLimit` entities are hit.

                let Interaction {
                    kind: InteractionKind::Hit(Hit { hit_limit, .. }),
                    ..
                } = ev.interaction;

                // If we contact multiple objects in *this* frame, when previously
                // there was 1 contact, and the hit limit is 2, then we should only hit 1
                // more at most.
                //
                // Also need to consider, if we have multiple `Interaction`s with different
                // `HitLimit`s (rare case?), interactions with a higher `HitLimit` should be
                // able to hit more objects.
                //
                // We need to count the sent `HitEvent`s for this case.
                let can_hit = match hit_limit {
                    HitLimit::Limit(limit) => {
                        // We use lesser or equal because the returned value is after the count has
                        // been incremented.
                        Self::update_hit_object_count(&mut hit_object_counts, entity_hitter)
                            <= limit
                    }
                    HitLimit::Unlimited => true,
                };

                can_hit
                    && match hit_repeat_trackerses.get(entity_hitter) {
                        Some(hit_repeat_trackers) => {
                            // If there is no clock, or the clock limit has been reached.
                            hit_repeat_trackers
                                .values()
                                .find(|hit_repeat_tracker| hit_repeat_tracker.entity == entity_hit)
                                .map(|hit_repeat_tracker| hit_repeat_tracker.clock.is_complete())
                                .unwrap_or(true)
                        }
                        None => true,
                    }
            })
            .map(|ev| HitEvent::new(ev.from, ev.to, ev.interaction.clone(), ev.body))
            .collect::<Vec<HitEvent>>();

        // Make sure we clear this before the next tick, otherwise the hitter can only hit more
        // objects when it has an `Interaction` with a larger `HitLimit`.
        hit_object_counts.clear();

        hit_ec.iter_write(hit_events);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.contact_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ContactEvent>>()
                .register_reader(),
        );
    }
}

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
    use object_status_model::config::StunPoints;
    use pretty_assertions::assert_eq;
    use shape_model::Volume;
    use slotmap::SlotMap;

    use super::HitDetectionSystem;

    const HIT_LIMIT: u32 = 3;

    #[test]
    fn inserts_hit_event_when_hit_repeat_trackers_does_not_exist() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_setup(setup_event_reader)
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
            .with_setup(setup_event_reader)
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
            .with_setup(setup_event_reader)
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
            .with_setup(setup_event_reader)
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
            .with_setup(setup_event_reader)
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
            InteractionKind::Hit(Hit::new(
                HitRepeatDelay::new(4),
                hit_limit,
                0,
                0,
                StunPoints::default(),
            )),
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
