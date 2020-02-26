//! User defined configuration types for UI labels.

pub use self::{
    ui_label::{UiLabel, UiLabelSystemData},
    ui_labels::UiLabels,
    ui_sprite_label::UiSpriteLabel,
    ui_sprite_labels::UiSpriteLabels,
};

mod ui_label;
mod ui_labels;
mod ui_sprite_label;
mod ui_sprite_labels;
