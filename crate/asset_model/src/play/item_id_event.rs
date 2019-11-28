use amethyst::ecs::Entity;

use crate::loaded::ItemId;

/// Event signalling a change in `ItemId`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemIdEvent {
    /// `ItemId` is inserted or modified for an entity.
    CreateOrUpdate {
        /// Entity whose `ItemId` changed.
        entity: Entity,
        /// The new `ItemId`.
        item_id: ItemId,
    },
}
