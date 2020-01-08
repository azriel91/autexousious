use amethyst::{
    ecs::{Entity, Read, System, SystemData, World, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use collision_model::{
    config::{Hit, HitRepeatDelay, Interaction, InteractionKind},
    play::{HitEvent, HitRepeatClock, HitRepeatTracker, HitRepeatTrackers},
};
use derive_new::new;
use slotmap::SlotMap;

/// Creates `HitRepeatTrackers`s for new `Hit` collisions.
///
/// This attaches `HitRepeatTrackers` to the entity with the `Interaction`.
#[derive(Debug, Default, new)]
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

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.hit_event_rid = Some(
            world
                .fetch_mut::<EventChannel<HitEvent>>()
                .register_reader(),
        );
    }
}
