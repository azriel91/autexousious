#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types to support building an application menu.



pub use self::event::MenuEvent;
pub use self::menu_item::MenuItem;

mod event;
mod menu_item;
