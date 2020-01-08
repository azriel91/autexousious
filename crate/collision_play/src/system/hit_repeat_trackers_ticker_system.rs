use amethyst::ecs::{Join, System, WriteStorage};
use collision_model::play::HitRepeatTrackers;
use derive_new::new;

/// Ticks each `HitRepeatTracker`'s clock.
#[derive(Debug, Default, new)]
pub struct HitRepeatTrackersTickerSystem;

type HitRepeatTrackersTickerSystemData<'s> = WriteStorage<'s, HitRepeatTrackers>;

impl<'s> System<'s> for HitRepeatTrackersTickerSystem {
    type SystemData = HitRepeatTrackersTickerSystemData<'s>;

    fn run(&mut self, mut hit_repeat_trackerses: Self::SystemData) {
        (&mut hit_repeat_trackerses)
            .join()
            .for_each(|hit_repeat_trackers| {
                hit_repeat_trackers
                    .values_mut()
                    .for_each(|hit_repeat_tracker| hit_repeat_tracker.clock.tick());

                hit_repeat_trackers
                    .retain(|_, hit_repeat_tracker| !hit_repeat_tracker.clock.is_complete());
            });
    } // kcov-ignore
}
