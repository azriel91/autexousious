use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component, Entity,
};
use derive_new::new;

/// ID of an item in the [`AssetWorld`].
///
/// An item is a "whole" object, so all item components associated with this item ID should be used
/// when spawning an entity in the game `World`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, new)]
pub struct ItemId(pub Entity);

impl Component for ItemId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
