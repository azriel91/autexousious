//! Configuration types for network joining.

pub use self::{
    network_join_event_command::NetworkJoinEventCommand, session_server_config::SessionServerConfig,
};

mod network_join_event_command;
mod session_server_config;
