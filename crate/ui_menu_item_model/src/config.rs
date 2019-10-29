//! User defined configuration types for UI menu items.

pub use self::{
    ui_menu_item::UiMenuItem, ui_menu_item_sequence_name::UiMenuItemSequenceName,
    ui_menu_items::UiMenuItems,
};

mod ui_menu_item;
mod ui_menu_item_sequence_name;
mod ui_menu_items;
