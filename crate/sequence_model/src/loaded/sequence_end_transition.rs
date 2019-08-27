use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use specs_derive::Component;

use crate::loaded::SequenceId;

/// Specifies the behaviour to transition when the sequence ends.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum SequenceEndTransition {
    /// Don't transition, stay on the last frame.
    #[derivative(Default)]
    None,
    /// Repeat the current sequence.
    Repeat,
    /// Delete the object after the sequence has ended.
    Delete,
    /// Transition to the specified sequence.
    SequenceId(SequenceId),
}
