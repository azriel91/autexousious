use serde::{Deserialize, Serialize};
use ui_label_model::config::UiLabel;

use crate::config::SessionDevicesWidget;

/// Configuration for initializing the session lobby UI.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionLobbyUi {
    /// Attributes of the session code label.
    pub session_code: UiLabel,
    /// List of session devices
    pub session_devices: SessionDevicesWidget,
}
