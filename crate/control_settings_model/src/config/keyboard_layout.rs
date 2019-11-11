use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Keyboard layout variants.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum KeyboardLayout {
    /// US keyboard layout
    #[derivative(Default)]
    Us,
}
