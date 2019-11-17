#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Loading logic for `UiMenuItem`s.

pub use crate::{ui_labels_loader::UiLabelsLoader, ui_sprite_labels_loader::UiSpriteLabelsLoader};

mod ui_labels_loader;
mod ui_sprite_labels_loader;
