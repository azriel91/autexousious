use amethyst::ecs::{storage::VecStorage, Component};
use specs_derive::Component;

/// Whether or not an object is charging.
#[derive(Clone, Component, Copy, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub enum ChargeStatus {
    /// Object is not charging.
    NotCharging,
    /// Object is undergoing initial charging delay.
    BeginDelay,
    /// Object is charging.
    Charging,
}
