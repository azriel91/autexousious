//! Configuration types for session hosting.

pub use self::{
    session_host_event_command::SessionHostEventCommand, session_server_config::SessionServerConfig,
};

mod session_host_event_command;
mod session_server_config;
