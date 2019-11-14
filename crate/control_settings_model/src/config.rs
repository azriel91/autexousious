//! Contains the types that represent the configuration on disk.

pub use self::{
    control_settings::ControlSettings, keyboard_layout::KeyboardLayout,
    keyboard_settings::KeyboardSettings,
};

mod control_settings;
mod keyboard_layout;
mod keyboard_settings;
