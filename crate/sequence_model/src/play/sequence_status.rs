use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;

/// Statuses that indicate whether a sequence has just begun, is ongoing, or has
/// ended.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum SequenceStatus {
    /// The sequence has just begun.
    #[derivative(Default)]
    Begin,
    /// The sequence began at least one tick ago, and has not yet reached the
    /// end.
    Ongoing,
    /// The sequence has ended.
    End,
}

/// Not every entity will have this, but since this is probably a `u8`, we don't
/// need an indirection table.
impl Component for SequenceStatus {
    type Storage = VecStorage<Self>;
}
