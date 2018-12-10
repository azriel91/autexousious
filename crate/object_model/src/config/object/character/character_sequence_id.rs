use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::config::object::SequenceId;

/// Object Sequence IDs.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
pub enum CharacterSequenceId {
    /// Default sequence for characters.
    #[derivative(Default)]
    Stand,
    /// Attack while standing.
    StandAttack,
    /// Walking sequence.
    Walk,
    /// Running sequence.
    Run,
    /// Running stop sequence.
    RunStop,
    /// Character is about to jump.
    Jump,
    /// Character has just jumped off the ground.
    JumpOff,
    /// Character is moving upwards from jumping.
    ///
    /// This is distinct from the `JumpDescend` state as this is when the jump velocity is
    /// effective, and characters may have different animations and attacks when moving upwards from
    /// a jump.
    JumpAscend,
    /// Character is descending from a jump.
    ///
    /// This sequence may also be used when the character has walked off a platform.
    JumpDescend,
    /// Character landed from jumping.
    JumpDescendLand,
    /// Character is hit while on ground.
    #[serde(rename = "flinch_0")]
    Flinch0,
    /// Character is hit while on ground, alternate sequence.
    #[serde(rename = "flinch_1")]
    Flinch1,
    /// Knocked off balance, moving upwards.
    FallForwardAscend,
    /// Knocked off balance, moving downwards.
    FallForwardDescend,
    /// Knocked off balance, landed on ground (bounce).
    FallForwardLand,
    /// Lying on ground face down.
    LieFaceDown,
}

/// Not every entity will have this, but since this is probably a `u8`, we don't need an indirection
/// table.
impl Component for CharacterSequenceId {
    type Storage = VecStorage<Self>;
}

impl SequenceId for CharacterSequenceId {}
