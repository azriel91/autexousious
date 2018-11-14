use std::fmt::Debug;

use amethyst::ecs::prelude::*;

use entity::{Position, Velocity};

/// Grouping of motion attributes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, new)]
pub struct Kinematics<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    /// Position of the entity.
    pub position: Position<S>,
    /// Velocity of the entity.
    pub velocity: Velocity<S>,
}

impl<S> Component for Kinematics<S>
where
    S: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    type Storage = VecStorage<Self>;
}
