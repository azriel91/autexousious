#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Menu to allow the user to select game mode.

pub use crate::{
    character_selection_ui_bundle::CharacterSelectionUiBundle,
    component::CharacterSelectionWidgetState,
    system::{
        CharacterSelectionInputSystem, CharacterSelectionInputSystemData,
        CharacterSelectionSfxSystem, CharacterSelectionSfxSystemData,
        CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetInputSystemData,
        CharacterSelectionWidgetUiSystem, CharacterSelectionWidgetUiSystemData,
    },
};

mod character_selection_ui_bundle;
mod component;
mod system;
