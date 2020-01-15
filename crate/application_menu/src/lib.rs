#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to support building an application menu.

pub use self::{
    component::{MenuItem, Siblings},
    event::MenuEvent,
    system::{
        MenuItemWidgetInputResources, MenuItemWidgetInputSystem, MenuItemWidgetInputSystemData,
    },
};

mod component;
mod event;
mod system;
