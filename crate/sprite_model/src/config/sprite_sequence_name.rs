use derivative::Derivative;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use typename_derive::TypeName;

/// Minimal `SequenceName` used as the default for `SpriteSequence`
#[derive(
    Clone,
    Copy,
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
pub enum SpriteSequenceName {
    /// Minimum variant.
    #[derivative(Default)]
    Unused,
}

impl SequenceName for SpriteSequenceName {}
