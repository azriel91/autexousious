#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to support building an application menu.

pub use self::{
    component::{MenuItem, MenuItemWidgetState, Siblings},
    event::MenuEvent,
    system::MenuItemWidgetInputSystem,
};

mod component;
mod event;
mod system;
