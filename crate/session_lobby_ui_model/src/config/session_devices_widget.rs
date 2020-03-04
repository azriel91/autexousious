use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};

use crate::config::SessionDeviceWidgetTemplate;

/// Configuration the widget to display all session devices.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct SessionDevicesWidget {
    /// Position of the widget.
    pub position: PositionInit,
    /// Widget template for displaying a session device.
    pub session_device_widget_template: SessionDeviceWidgetTemplate,
}
