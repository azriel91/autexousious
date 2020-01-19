use std::num::NonZeroIsize;

use serde::{Deserialize, Serialize};

/// Direction to switch asset selection.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AssetSwitch {
    /// Switch to previous asset.
    Previous,
    /// Switch to next asset.
    Next,
    /// Switch to asset `n` away.
    Skip(NonZeroIsize),
}
