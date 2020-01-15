use derive_new::new;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

/// Template for initializing an `AssetSelectionHighlight`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct AshTemplate {
    /// Sprite to display.
    #[serde(flatten)]
    pub sprite: UiSpriteLabel,
}
