#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Custom events to control states.
//!
//! This crate provides the top level enum type that all states receive as their
//! custom event.
//!
//! # Usage
//!
//! ## Event Senders
//!
//! Event senders should only send events with variants for the active state. If
//! there is an error in the processing of any event, make sure to check that
//! both the sender and the receiving state are honouring the contract of the
//! events.
//!
//! ## States
//!
//! States should handle its own event variant and log an error with the event
//! on other states' variants, as this indicates that one of the following is
//! true:
//!
//! * There is a sender that incorrectly assumes the state will transition, and
//!   it sent in the following state's event type.
//! * The state *should* have transitioned, but did not.
//! * There are multiple event senders that are not catering for each other.

pub use crate::app_event::{AppEvent, AppEventReader, AppEventVariant};

mod app_event;
