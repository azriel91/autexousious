#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Handles resources common to an application's UI.
//!
//! Currently this just registers fonts with the world. In the future, this crate may also handle
//! switching between themes and internationalization.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate ron;
#[macro_use]
extern crate serde;

pub use bundle::ApplicationUiBundle;
pub use font_config::FontConfig;
pub use font_variant::FontVariant;

mod bundle;
mod font_config;
mod font_variant;
