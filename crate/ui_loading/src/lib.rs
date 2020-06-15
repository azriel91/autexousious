#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes UI configuration into the loaded UI model.

pub use self::ui_ascl::{
    UiAscl, UiAsclCharacterSelection, UiAsclComponents, UiAsclControlSettings, UiAsclForm,
    UiAsclMapSelection, UiAsclMenu, UiAsclSessionLobby,
};
pub use crate::ui_loading_bundle::UiLoadingBundle;

mod ui_ascl;
mod ui_loading_bundle;
