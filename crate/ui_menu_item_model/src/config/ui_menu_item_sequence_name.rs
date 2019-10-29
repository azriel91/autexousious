use derivative::Derivative;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use typename_derive::TypeName;

/// `UiMenuItem` sequence names.
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
pub enum UiMenuItemSequenceName {
    /// Item is not focused.
    #[derivative(Default)]
    Inactive,
    /// Item is focused.
    Active,
    /// Item has been selected.
    Selected,
}

impl SequenceName for UiMenuItemSequenceName {}
