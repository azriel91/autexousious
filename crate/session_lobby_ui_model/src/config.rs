//! User defined configuration types for the session lobby UI.

pub use self::{
    session_device_widget_template::SessionDeviceWidgetTemplate,
    session_devices_widget::SessionDevicesWidget, session_lobby_ui::SessionLobbyUi,
};

mod session_device_widget_template;
mod session_devices_widget;
mod session_lobby_ui;
