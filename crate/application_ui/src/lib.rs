#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Handles resources common to an application's UI.
//!
//! Currently this just registers fonts with the world. In the future, this crate may also handle
//! switching between themes and internationalization.
//!
//! # Usage
//!
//! ## Font Configuration
//!
//! This bundle expects to find a `resources/font_config.ron` file next to the executable. The
//! configuration format is as follows:
//!
//! ```rust,ignore
//! (
//!     regular: "relative/path/to/regular.ttf",
//!     bold: "relative/path/to/bold.ttf",
//!     italic: "relative/path/to/italic.ttf",
//!     bold_italic: "relative/path/to/bold_italic.ttf",
//! )
//! ```
//!
//! The paths are relative to the `assets` directory next to the executable. Visually, the directory
//! structure is as follows:
//!
//! ```text
//! bin
//! ├── resources
//! │  ├── font_config.ron
//! │  └── ...
//! ├── assets
//! │   └── relative
//! │      └── path
//! │         ├── to
//! │         │  ├── regular.ttf
//! │         │  ├── bold.ttf
//! │         │  ├── it.ttf
//! │         │  └── boldit.ttf
//! │         └── ...
//! ├── my_app.exe
//! └── ...
//! ```
//!
//! # Examples
//!
//! For the code example, please see the `01_draw_text` example in this repository, which renders
//! text in regular, bold, italic, and bold italic fonts.

#[macro_use]
extern crate application;
#[macro_use]
extern crate derive_more;
#[cfg(test)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
use strum;
#[macro_use]
extern crate strum_macros;

pub use crate::{
    font_config::FontConfig, font_variant::FontVariant, theme::Theme, theme_loader::ThemeLoader,
};

mod font_config;
mod font_variant;
mod theme;
mod theme_loader;
