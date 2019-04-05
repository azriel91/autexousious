use amethyst::{
    ecs::{Read, ReadStorage, Resources, System, SystemData, Write},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{
    config::{Hit, HitLimit, Interaction, InteractionKind},
    play::{CollisionEvent, HitEvent, HitRepeatTrackers},
};
use derive_new::new;

use typename_derive::TypeName;

/// Detects whether a `HitEvent` occurs there is collision between an `Interaction` and a `Volume`.
#[derive(Debug, Default, TypeName, new)]
pub struct HitDetectionSystem {
    /// Reader ID for the `CollisionEvent` event channel.
    #[new(default)]
    collision_event_rid: Option<ReaderId<CollisionEvent>>,
}

type HitDetectionSystemData<'s> = (
    Read<'s, EventChannel<CollisionEvent>>,
    ReadStorage<'s, HitRepeatTrackers>,
    Write<'s, EventChannel<HitEvent>>,
);

impl<'s> System<'s> for HitDetectionSystem {
    type SystemData = HitDetectionSystemData<'s>;

    fn run(&mut self, (collision_ec, hit_repeat_trackerses, mut hit_ec): Self::SystemData) {
        let hit_events = collision_ec
            .read(
                self.collision_event_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for HitDetectionSystem."),
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

                match hit_repeat_trackerses.get(entity_hitter) {
                    Some(hit_repeat_trackers) => {
                        // FIXME: Incorrect limit enforcement.
                        //
                        // This is wrong because this means if we collide with multiple objects in
                        // *this* frame, when previously there was 1 collision, and the hit limit is
                        // 2, then we should only hit 1 more at most. Currently this would hit all
                        // new objects.
                        //
                        // Also need to consider, if we have multiple `Interaction`s with different
                        // `HitLimit`s (rare case?), interactions with a higher `HitLimit` should be
                        // able to hit more objects.
                        //
                        // We need to count the sent `HitEvent`s for this case.
                        let can_hit = match hit_limit {
                            HitLimit::Limit(limit) => (hit_repeat_trackers.len() as u32) < limit,
                            HitLimit::Unlimited => true,
                        };

                        // If there is no clock, or the clock limit has been reached.
                        can_hit
                            && hit_repeat_trackers
                                .iter()
                                .find(|hit_repeat_tracker| hit_repeat_tracker.entity == entity_hit)
                                .map(|hit_repeat_tracker| hit_repeat_tracker.clock.is_complete())
                                .unwrap_or(true)
                    }

                    // FIXME: Incorrect limit enforcement.
                    //
                    // We need to count the sent `HitEvent`s for this case.
                    None => true,
                }
            })
            .map(|ev| HitEvent::new(ev.from, ev.to, ev.interaction.clone(), ev.body))
            .collect::<Vec<HitEvent>>();
        hit_ec.iter_write(hit_events);
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.collision_event_rid = Some(
            res.fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind},
        play::{CollisionEvent, HitEvent, HitRepeatClock, HitRepeatTracker, HitRepeatTrackers},
    };
    use logic_clock::LogicClock;
    use shape_model::Volume;

    use super::HitDetectionSystem;

    #[test]
    fn inserts_hit_event_when_hit_repeat_trackers_does_not_exist() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitDetectionSystem::new(), "", &[])
            .with_setup(setup_event_reader)
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                send_event(world, collision_event(entity_from, entity_to));

                world.add_resource((entity_from, entity_to));
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
                send_event(world, collision_event(entity_from, entity_to));

                world.add_resource((entity_from, entity_to));
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
                send_event(world, collision_event(entity_from, entity_to));

                world.add_resource((entity_from, entity_to));
            })
            .with_assertion(|world| {
                assert_events(world, vec![]);
            })
            .run()
    }

    fn setup_event_reader(world: &mut World) {
        let hit_event_rid = world
            .write_resource::<EventChannel<HitEvent>>()
            .register_reader();

        world.add_resource(hit_event_rid);
    }

    fn send_event(world: &mut World, event: CollisionEvent) {
        let mut ec = world.write_resource::<EventChannel<CollisionEvent>>();
        ec.single_write(event)
    }

    fn hit_repeat_trackers(entity_to: Entity) -> HitRepeatTrackers {
        HitRepeatTrackers::new(vec![HitRepeatTracker::new(
            entity_to,
            HitRepeatClock::new(LogicClock::new(4)),
        )])
    }

    fn collision_event(entity_from: Entity, entity_to: Entity) -> CollisionEvent {
        CollisionEvent::new(
            entity_from,
            entity_to,
            interaction(HitLimit::Limit(2)),
            body(),
        )
    }

    fn hit_event(entity_from: Entity, entity_to: Entity) -> HitEvent {
        HitEvent::new(
            entity_from,
            entity_to,
            interaction(HitLimit::Limit(2)),
            body(),
        )
    }

    fn interaction(hit_limit: HitLimit) -> Interaction {
        Interaction::new(
            InteractionKind::Hit(Hit::new(HitRepeatDelay::new(4), hit_limit, 0, 0)),
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
