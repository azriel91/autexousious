#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! This crate provides types to support automation of operations in an Amethyst application.
//!
//! One of the main use cases is automated testing. The types allow input to the application, which
//! can control it as a replacement for device input (e.g. keyboard, mouse).

pub use crate::{
    intercept::{
        ApplicationEventIntercept, FixedTimeoutIntercept, Intercept, KeyboardEscapeIntercept,
    },
    state::RobotState,
};

mod intercept;
mod state;
