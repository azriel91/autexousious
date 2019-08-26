use derivative::Derivative;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use typename_derive::TypeName;

/// `Energy` sequence names.
#[derive(
    Clone,
    Debug,
    Derivative,
    Deserialize,
    Display,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    TypeName,
)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EnergySequenceName {
    /// Default sequence for energies.
    #[derivative(Default)]
    Hover,
    /// Sequence to switch to when hitting another object.
    Hitting,
    /// Sequence to switch to when hit by another object.
    Hit,
}

impl SequenceName for EnergySequenceName {}
