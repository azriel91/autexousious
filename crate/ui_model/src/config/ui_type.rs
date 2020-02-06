use application_menu::MenuIndex;
use character_selection_ui_model::config::CharacterSelectionUi;
use control_settings_model::config::ControlSettings;
use map_selection_ui_model::config::MapSelectionUi;
use serde::{Deserialize, Serialize};
use ui_menu_item_model::config::UiMenuItems;

/// UI types -- generic menu, character selection, map selection.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum UiType {
    /// Generic menu UI.
    Menu(UiMenuItems<MenuIndex>),
    /// Character selection UI.
    CharacterSelection(CharacterSelectionUi),
    /// Map selection UI.
    MapSelection(MapSelectionUi),
    /// Control Settings UI.
    ControlSettings(ControlSettings),
}
