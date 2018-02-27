//! Module that provides a simple menu.

pub use self::bundle::MenuBundle;
pub use self::event::MenuEvent;
pub use self::menu_item::MenuItem;
pub use self::system::UiEventHandlerSystem;

pub mod main_menu;

mod bundle;
mod event;
mod menu_item;
mod system;
