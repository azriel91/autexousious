use asset_derive::Asset;
use game_mode_selection_model::GameModeIndex;
use serde::{Deserialize, Serialize};
use ui_menu_item_model::config::UiMenuItems;

/// UI types -- generic menu, character selection, map selection.
#[derive(Asset, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum UiType {
    /// Generic menu UI.
    Menu(UiMenuItems<GameModeIndex>),
}
