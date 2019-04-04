use amethyst::ecs::Entity;
use derive_new::new;

use crate::play::ImpactRepeatClock;

/// Stores the hit `Entity` and the `ImpactRepeatClock` for cooldown period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct ImpactRepeatTracker {
    /// Object entity that was hit.
    pub entity: Entity,
    /// Logic clock to track that enough ticks have passed.
    ///
    /// When this clock has reached its limit, then another impact interaction from the attacking
    /// entity may hit the target entity.
    pub clock: ImpactRepeatClock,
}
