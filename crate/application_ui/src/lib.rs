#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Handles resources common to an application's UI.
//!
//! Currently this just registers a font with the world. In the future, this crate may also handle
//! switching between themes and internationalization.

extern crate amethyst;

pub use bundle::ApplicationUiBundle;

mod bundle;
