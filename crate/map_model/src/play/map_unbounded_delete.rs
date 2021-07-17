use amethyst::ecs::{storage::NullStorage, Component};

/// Marks an entity that should be deleted when out of map bounds.
///
/// This is explicitly a component instead of the absence of `MapBounded` as
/// there can be entities that are out of bounds but should not be deleted.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct MapUnboundedDelete;
