use network_session_model::play::SessionDevice;

use crate::play::NetData;

/// `SessionDevice` received over a network connection.
pub type NetSessionDevice = NetData<SessionDevice>;
