use std::marker::PhantomData;

use derive_new::new;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

/// Highlights an asset selection.
///
/// # Type Parameters
///
/// * `T`: Type of the asset to highlight.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct AssetSelectionHighlight<T> {
    /// Sprite to display.
    #[serde(flatten)]
    pub sprite: UiSpriteLabel,
    /// Marker.
    #[serde(skip)]
    pub marker: PhantomData<T>,
}
