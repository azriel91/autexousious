use amethyst::ecs::Entity;
use derive_new::new;
use enumflags2::BitFlags;

use crate::play::BoundaryFace;

/// Stores the entity and boundary faces of a `MapBoundaryEvent`.
#[derive(Clone, Copy, Debug, PartialEq, new)]
pub struct MapBoundaryEventData {
    /// Entity that crossed the map boundary.
    pub entity: Entity,
    /// Boundary faces that were crossed.
    pub boundary_faces: BitFlags<BoundaryFace>,
}
