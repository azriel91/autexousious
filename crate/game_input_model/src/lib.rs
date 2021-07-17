#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used for control input.
//!
//! The `GameInputEvent` is a necessary distinct type from
//! `InputEvent<ControlBindings>` because for online play, local input needs to
//! be transmitted to the session server to be broadcast to other clients in the
//! session, but remote input that is received needs to be processed and not
//! re-broadcast.
//!
//! `ControlInputEvent`s are not suitable to be serialized / deserialized as the
//! event data contains `Entity` values, which will be different across clients.
//!
//! To satisfy this process, `InputEvent<ControlBindings>` will be used to
//! construct `GameInputEvent`s, and subsequently `ControlInputEvent`s.

pub use crate::game_input_event::GameInputEvent;

pub mod config;
pub mod loaded;
pub mod play;

mod game_input_event;
