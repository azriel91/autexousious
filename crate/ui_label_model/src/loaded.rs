//! Types that represent processed configuration.

pub use self::{
    asset_ui_labels::AssetUiLabels, asset_ui_sprite_labels::AssetUiSpriteLabels,
    ui_sprite_label::UiSpriteLabel, ui_sprite_labels::UiSpriteLabels,
};

mod asset_ui_labels;
mod asset_ui_sprite_labels;
mod ui_sprite_label;
mod ui_sprite_labels;
