//! Main menu module.

pub use self::{
    index::Index, main_menu_state::MainMenuState, ui_event_handler_system::UiEventHandlerSystem,
};

mod index;
mod main_menu_state;
mod ui_event_handler_system;
