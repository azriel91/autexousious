#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for UI SPI providers used at runtime.

pub use crate::{
    ui_rectify_system_data::UiRectifySystemData, ui_widget_rectifier::UiWidgetRectifier,
};

mod ui_rectify_system_data;
mod ui_widget_rectifier;
