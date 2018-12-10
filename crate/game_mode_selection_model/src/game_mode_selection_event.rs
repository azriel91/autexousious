use application_menu::MenuEvent;

use crate::GameModeIndex;

/// Event indicating game mode selection.
pub type GameModeSelectionEvent = MenuEvent<GameModeIndex>;
