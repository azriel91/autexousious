#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides a custom event channel for applications.
//!
//! Amethyst does not yet support custom event types for `State`s to handle.
//! This is a workaround.
//!
//! # Development
//!
//! At time of writing, we are using a pre-0.6 version of amethyst. This
//! version's event system has an Event enum with the following variants:
//!
//! * `WindowEvent`: variant that eventually backs onto winit's `WindowEvent`
//! * `DeviceEvent`: variant that is meant to represent hardware input devices
//! * Awakened
//!
//! There is no variant for user events. This is what `@torkleyy` said when I
//! asked:
//!
//! > We didn't reach any consensus on such user-made events yet. The current
//! event types are just > reexport of winit types, which we intend to replace.
//!
//! See:
//!
//! * <https://github.com/amethyst/amethyst/issues/229>
//! * <https://github.com/amethyst/amethyst/issues/481>
//!
//! The following documentation was useful while writing this
//!
//! * <https://docs.rs/specs/latest/specs/struct.World.html>
//! * <https://docs.rs/shred/latest/shred/>
//! * <https://docs.rs/shrev/latest/shrev/struct.EventChannel.html>
//! * <https://www.amethyst.rs/doc/master/doc/amethyst/enum.StateEvent.html>

pub use crate::event::ApplicationEvent;

mod event;
