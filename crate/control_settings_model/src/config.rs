//! Contains the types that represent the configuration on disk.

pub use self::{
    control_button_label::ControlButtonLabel, control_button_labels::ControlButtonLabels,
    control_settings::ControlSettings, keyboard_layout::KeyboardLayout,
    keyboard_settings::KeyboardSettings,
};

mod control_button_label;
mod control_button_labels;
mod control_settings;
mod keyboard_layout;
mod keyboard_settings;
