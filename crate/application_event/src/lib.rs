#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Custom events to control states.
//!
//! This crate provides the top level enum type that all states receive as their custom event.
//!
//! # Usage
//!
//! ## Event Senders
//!
//! Event senders should only send events with variants for the active state. If there is an error
//! in the processing of any event, make sure to check that both the sender and the receiving state
//! are honouring the contract of the events.
//!
//! ## States
//!
//! States should handle its own event variant and log an error with the event on other states'
//! variants, as this indicates that one of the following is true:
//!
//! * There is a sender that incorrectly assumes the state will transition, and it sent in the
//!   following state's event type.
//! * The state *should* have transitioned, but did not.
//! * There are multiple event senders that are not catering for each other.

#[macro_use]
extern crate amethyst;
extern crate character_selection_model;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_more;
extern crate game_mode_selection_model;
extern crate map_selection_model;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub use app_event::{AppEvent, AppEventReader, AppEventVariant};
pub use from_app_event::FromAppEvent;

mod app_event;
mod from_app_event;
