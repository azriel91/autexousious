use amethyst::ecs::{storage::VecStorage, Component};

use crate::play::{IndependentCounter, TeamCounter};

/// Represents the in-game grouping of player teams.
#[derive(Clone, Component, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
#[storage(VecStorage)]
pub enum Team {
    /// Independent team.
    ///
    /// This variant is intended for multiple players to be on different teams,
    /// even though their team sigil (or colour) may be rendered the same
    /// (neutral).
    Independent(IndependentCounter),
    /// Numbered team.
    ///
    /// This variant is intended for teams whose team sigil (or colour) should
    /// be rendered consistently.
    Number(TeamCounter),
}
