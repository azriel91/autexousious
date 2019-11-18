use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Boundary of the playable area of the map.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, new)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct MapBounds {
    /// X coordinate of the map's left boundary.
    pub x: u32,
    /// Y coordinate of the map's bottom boundary.
    pub y: u32,
    /// Z coordinate of the map's back boundary.
    pub z: u32,
    /// Distance that the map extends to the right.
    pub width: u32,
    /// Distance that the map extends upwards.
    pub height: u32,
    /// Distance that the map extends forwards (projected down).
    pub depth: u32,
}
