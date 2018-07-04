use amethyst::ecs::prelude::*;

use entity::{Position, Velocity};

/// Grouping of motion attributes.
#[derive(Clone, Copy, Constructor, Debug, Default, PartialEq, Eq)]
pub struct Kinematics<S>
where
    S: Send + Sync + 'static,
{
    /// Position of the entity.
    pub position: Position<S>,
    /// Velocity of the entity.
    pub velocity: Velocity<S>,
}

impl<S> Component for Kinematics<S>
where
    S: Send + Sync + 'static,
{
    type Storage = VecStorage<Self>;
}
