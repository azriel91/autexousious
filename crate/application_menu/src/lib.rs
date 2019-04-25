#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to support building an application menu.

pub use self::{event::MenuEvent, menu_item::MenuItem};

mod event;
mod menu_item;
