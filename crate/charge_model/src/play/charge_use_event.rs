use amethyst::ecs::Entity;
use derive_new::new;

use crate::config::ChargePoints;

/// Event indicating `ChargePoints` are used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct ChargeUseEvent {
    /// Entity that used the `ChargePoints`.
    pub entity: Entity,
    /// `ChargePoints` used.
    pub charge_points: ChargePoints,
}
