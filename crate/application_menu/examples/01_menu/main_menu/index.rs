use amethyst::prelude::*;

use other::OtherState;

/// Indicies of main menu items.
#[derive(Debug, Clone, Copy)]
pub enum Index {
    /// Menu item for starting a game.
    StartGame,
    /// Menu item for exiting the application.
    Exit,
}

impl Index {
    /// Returns a human readable string of this menu item.
    pub fn title(&self) -> &str {
        match *self {
            Index::StartGame => "Start Game",
            Index::Exit => "Exit",
        }
    }

    pub fn trans<'a, 'b>(&self) -> Trans<GameData<'a, 'b>> {
        match *self {
            Index::StartGame => Trans::Push(Box::new(OtherState::new())),
            Index::Exit => Trans::Quit,
        }
    }
}
