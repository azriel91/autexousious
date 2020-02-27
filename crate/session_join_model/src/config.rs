//! Configuration types for session joining.

pub use self::{
    session_join_event_command::SessionJoinEventCommand, session_server_config::SessionServerConfig,
};

mod session_join_event_command;
mod session_server_config;
