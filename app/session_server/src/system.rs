//! Systems exclusive to the session server.
//!
//! Not yet sure how to structure the repository:
//!
//! * We don't want server crates to depend on `amethyst` with the `"renderer"`
//!   feature.
//! * Crates under `crate` are configured to use a consistent set of `amethyst`
//!   features.

pub use self::{
    network_input_responder_system::{
        NetworkInputResponderSystem, NetworkInputResponderSystemDesc,
    },
    session_cleaner::SessionCleaner,
    session_device_disconnect_responder_system::{
        SessionDeviceDisconnectResponderSystem, SessionDeviceDisconnectResponderSystemDesc,
    },
    session_host_responder_system::{SessionHostResponderSystem, SessionHostResponderSystemDesc},
    session_join_responder_system::{SessionJoinResponderSystem, SessionJoinResponderSystemDesc},
    session_lobby_responder_system::{
        SessionLobbyResponderSystem, SessionLobbyResponderSystemDesc,
    },
    session_message_responder_system::{
        SessionMessageResponderSystem, SessionMessageResponderSystemDesc,
    },
};

mod network_input_responder_system;
mod session_cleaner;
mod session_device_disconnect_responder_system;
mod session_host_responder_system;
mod session_join_responder_system;
mod session_lobby_responder_system;
mod session_message_responder_system;
