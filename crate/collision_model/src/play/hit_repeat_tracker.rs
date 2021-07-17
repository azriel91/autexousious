use amethyst::ecs::Entity;
use derive_new::new;

use crate::play::HitRepeatClock;

/// Stores the hit `Entity` and the `HitRepeatClock` for cooldown period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct HitRepeatTracker {
    /// Object entity that was hit.
    pub entity: Entity,
    /// Logic clock to track that enough ticks have passed.
    ///
    /// When this clock has reached its limit, then another hit interaction from
    /// the attacking entity may hit the target entity.
    pub clock: HitRepeatClock,
}
