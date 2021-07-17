#![allow(clippy::nonstandard_macro_braces)] // TODO: Pending https://github.com/rust-lang/rust-clippy/issues/7434

use serde::{Deserialize, Serialize};

use crate::config::{AshTemplate, AssetDisplay};

/// Displays available assets and highlights selected asset.
///
/// # Type Parameters
///
/// * `T`: Type to indicate the assets to display.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetSelector<T> {
    /// Displays assets.
    #[serde(flatten)]
    pub asset_display: AssetDisplay<T>,
    /// Highlights the selected asset.
    pub selection_highlights: Vec<AshTemplate>,
}
