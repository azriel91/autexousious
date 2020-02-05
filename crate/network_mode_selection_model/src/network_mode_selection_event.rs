use application_menu::MenuEvent;

use crate::NetworkModeIndex;

/// Event indicating game mode selection.
pub type NetworkModeSelectionEvent = MenuEvent<NetworkModeIndex>;
