use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component,
};
use derivative::Derivative;
use sequence_model::config::SequenceId;
use serde::{Deserialize, Serialize};

/// Energy sequence IDs.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
pub enum EnergySequenceId {
    /// Default sequence for energies.
    #[derivative(Default)]
    Hover,
}

impl Component for EnergySequenceId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl SequenceId for EnergySequenceId {}
