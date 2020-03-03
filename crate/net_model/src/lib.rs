#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used for sending / receiving information over the network.
//!
//! This is an umbrella crate -- currently `LaminarNetworkBundle` only supports sending / receiving
//! messages from one socket for the entire application, so messages from different concerns are
//! bundled together under one enum.
//!
//! An improvement would be to open separate sockets for session joining vs gameplay messages.

pub mod play;
