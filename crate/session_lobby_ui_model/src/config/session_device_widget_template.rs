use serde::{Deserialize, Serialize};
use ui_label_model::config::UiLabel;
use ui_model_spi::config::Dimensions;

/// Configuration for displaying information about a particular session device.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct SessionDeviceWidgetTemplate {
    /// Dimensions of the widget.
    ///
    /// Each session device widget will be placed below the previous one. The
    /// width is currently not used.
    pub dimensions: Dimensions,
    /// Label attributes for the `SessionDeviceId`.
    pub device_id: UiLabel,
    /// Label attributes for the `SessionDeviceName`.
    pub device_name: UiLabel,
}
