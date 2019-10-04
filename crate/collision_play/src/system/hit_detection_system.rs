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
