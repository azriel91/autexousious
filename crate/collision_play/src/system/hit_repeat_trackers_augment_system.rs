use amethyst::{
    ecs::{Entity, Read, Resources, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{
    config::{Hit, HitRepeatDelay, Interaction, InteractionKind},
    play::{HitEvent, HitRepeatClock, HitRepeatTracker, HitRepeatTrackers},
};
use derive_new::new;
use slotmap::SlotMap;
use typename_derive::TypeName;

/// Creates `HitRepeatTrackers`s for new `Hit` collisions.
///
/// This attaches `HitRepeatTrackers` to the entity with the `Interaction`.
#[derive(Debug, Default, TypeName, new)]
pub struct HitRepeatTrackersAugmentSystem {
    /// Reader ID for the `HitEvent` event channel.
    #[new(default)]
    hit_event_rid: Option<ReaderId<HitEvent>>,
}

type HitRepeatTrackersAugmentSystemData<'s> = (
    Read<'s, EventChannel<HitEvent>>,
    WriteStorage<'s, HitRepeatTrackers>,
);

impl HitRepeatTrackersAugmentSystem {
    fn hit_repeat_tracker(entity_to: Entity, repeat_delay: HitRepeatDelay) -> HitRepeatTracker {
        let hit_repeat_clock = HitRepeatClock::new(*repeat_delay as usize);
        HitRepeatTracker::new(entity_to, hit_repeat_clock)
    }
}

impl<'s> System<'s> for HitRepeatTrackersAugmentSystem {
    type SystemData = HitRepeatTrackersAugmentSystemData<'s>;

    fn run(&mut self, (collision_ec, mut hit_repeat_trackerses): Self::SystemData) {
        // Read from channel
        collision_ec
            .read(
                self.hit_event_rid
                    .as_mut()
                    .expect("Expected reader ID to exist for HitRepeatTrackersAugmentSystem."),
            )
            .for_each(|ev| {
                // Only add trackers for `Hit` interactions.
                let Interaction {
                    kind: InteractionKind::Hit(Hit { repeat_delay, .. }),
                    ..
                } = ev.interaction;

                // This assumes `ev.to` is the hit object entity. If we have a separate
                // entity for each `Body`, then this assumption breaks, and we need to
                // traverse the entity hierarchy to find the object entity.
                let hit_object = ev.to;

                match hit_repeat_trackerses.get_mut(ev.from) {
                    Some(hit_repeat_trackers) => {
                        if hit_repeat_trackers
                            .values()
                            .all(|hit_repeat_tracker| hit_repeat_tracker.entity != hit_object)
                        {
                            let hit_repeat_tracker =
                                Self::hit_repeat_tracker(hit_object, repeat_delay);
                            hit_repeat_trackers.insert(hit_repeat_tracker);
                        }
                    }
                    None => {
                        let hit_repeat_tracker = Self::hit_repeat_tracker(hit_object, repeat_delay);
                        let mut slot_map = SlotMap::new();
                        slot_map.insert(hit_repeat_tracker);
                        let hit_repeat_trackers = HitRepeatTrackers::new(slot_map);
                        hit_repeat_trackerses
                            .insert(ev.from, hit_repeat_trackers)
                            .expect("Failed to insert `HitRepeatTrackers`.");
                    }
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.hit_event_rid = Some(res.fetch_mut::<EventChannel<HitEvent>>().register_reader());
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use collision_model::{
        config::{Hit, HitLimit, HitRepeatDelay, Interaction, InteractionKind},
        play::{HitEvent, HitRepeatClock, HitRepeatTracker, HitRepeatTrackers},
    };
    use object_status_model::config::StunPoints;
    use shape_model::Volume;
    use slotmap::SlotMap;

    use super::HitRepeatTrackersAugmentSystem;

    #[test]
    fn inserts_hit_repeat_trackers_for_hitter() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitRepeatTrackersAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to, interaction(), body());
                send_event(world, event);

                world.add_resource((entity_from, entity_to));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to) = *world.read_resource::<(Entity, Entity)>();
                let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses.get(entity_from);

                let mut slot_map = SlotMap::new();
                slot_map.insert(HitRepeatTracker::new(entity_to, HitRepeatClock::new(4)));
                assert_eq!(Some(&HitRepeatTrackers::new(slot_map)), hit_repeat_trackers);
            })
            .run()
    }

    #[test]
    fn inserts_hit_repeat_tracker_for_different_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(HitRepeatTrackersAugmentSystem::new(), "", &[])
            .with_effect(|world| {
                let entity_from = world.create_entity().build();
                let entity_to_0 = world.create_entity().build();
                let entity_to_1 = world.create_entity().build();

                let event = HitEvent::new(entity_from, entity_to_0, interaction(), body());
                send_event(world, event);

                world.add_resource((entity_from, entity_to_0, entity_to_1));
            })
            .with_assertion(|world| {
                let (entity_from, entity_to_0, _entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();
                let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses.get(entity_from);

                let mut slot_map = SlotMap::new();
                slot_map.insert(HitRepeatTracker::new(entity_to_0, HitRepeatClock::new(4)));
                assert_eq!(Some(&HitRepeatTrackers::new(slot_map)), hit_repeat_trackers);
            })
            .with_effect(|world| {
                let (entity_from, _entity_to_0, entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();

                let event = HitEvent::new(entity_from, entity_to_1, interaction(), body());
                send_event(world, event);
            })
            .with_assertion(|world| {
                let (entity_from, entity_to_0, entity_to_1) =
                    *world.read_resource::<(Entity, Entity, Entity)>();
                let hit_repeat_trackerses = world.read_storage::<HitRepeatTrackers>();
                let hit_repeat_trackers = hit_repeat_trackerses.get(entity_from);

                let mut slot_map = SlotMap::new();
                slot_map.insert(HitRepeatTracker::new(entity_to_0, HitRepeatClock::new(4)));
                slot_map.insert(HitRepeatTracker::new(entity_to_1, HitRepeatClock::new(4)));
                assert_eq!(Some(&HitRepeatTrackers::new(slot_map)), hit_repeat_trackers);
            })
            .run()
    }

    fn send_event(world: &mut World, event: HitEvent) {
        let mut ec = world.write_resource::<EventChannel<HitEvent>>();
        ec.single_write(event)
    } // kcov-ignore

    fn interaction() -> Interaction {
        Interaction::new(
            InteractionKind::Hit(Hit::new(
                HitRepeatDelay::new(4),
                HitLimit::Unlimited,
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
}
