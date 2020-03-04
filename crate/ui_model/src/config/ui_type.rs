use application_menu::MenuIndex;
use character_selection_ui_model::config::CharacterSelectionUi;
use control_settings_model::config::ControlSettings;
use map_selection_ui_model::config::MapSelectionUi;
use serde::{Deserialize, Serialize};
use session_lobby_ui_model::config::SessionLobbyUi;
use ui_form_model::config::UiFormItems;
use ui_menu_item_model::config::UiMenuItems;

/// UI types -- generic menu, character selection, map selection.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum UiType {
    /// Character selection UI.
    CharacterSelection(CharacterSelectionUi),
    /// Control Settings UI.
    ControlSettings(ControlSettings),
    /// Generic menu UI.
    Menu(UiMenuItems<MenuIndex>),
    /// Generic form UI.
    Form(UiFormItems),
    /// Map selection UI.
    MapSelection(MapSelectionUi),
    /// Session Lobby UI.
    ///
    /// This is used for both hosts and joiners.
    SessionLobby(SessionLobbyUi),
}
