//! Systems exclusive to the session server.
//!
//! Not yet sure how to structure the repository:
//!
//! * We don't want server crates to depend on `amethyst` with the `"renderer"` feature.
//! * Crates under `crate` are configured to use a consistent set of `amethyst` features.

pub use self::session_join_responder_system::SessionJoinResponderSystem;

mod session_join_responder_system;
