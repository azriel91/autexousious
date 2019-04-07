use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;

/// State that tracks an object's attachment to the surrounding environment.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum Grounding {
    /// Object is resting on the ground, whether it is the floor or another solid object.
    #[derivative(Default)]
    OnGround,
    /// Object is in the air.
    Airborne,
}

/// Not every entity will have this, but since this is probably a `u8`, we don't need an indirection
/// table.
impl Component for Grounding {
    type Storage = VecStorage<Self>;
}
