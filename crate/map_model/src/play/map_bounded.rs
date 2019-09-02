use amethyst::ecs::{storage::NullStorage, Component};
use specs_derive::Component;

/// Marks an entity as bounded to map boundaries.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct MapBounded;
