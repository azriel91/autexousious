use amethyst::prelude::Trans;

/// Game mode menu indicies.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    } // kcov-ignore

    /// Returns the transition when this index has been selected.
    pub fn trans(&self) -> Trans {
        match *self {
            // TODO: Trans::Push(Box::new(game_play::State::new())),
            Index::StartGame => Trans::None,
            Index::Exit => Trans::Quit,
        }
    } // kcov-ignore
}

#[cfg(test)]
mod test {
    use amethyst::prelude::*;

    use super::Index;

    #[test]
    fn start_game_title() {
        assert_eq!("Start Game", Index::StartGame.title());
    }

    #[test]
    fn exit_title() {
        assert_eq!("Exit", Index::Exit.title());
    }

    #[test]
    fn start_game_trans_returns_none() {
        match Index::StartGame.trans() {
            Trans::None => {}
            _ => panic!("Expected Trans::None"), // kcov-ignore
        }
    }

    #[test]
    fn exit_trans_returns_quit() {
        match Index::Exit.trans() {
            Trans::Quit => {}
            _ => panic!("Expected Trans::Quit"), // kcov-ignore
        }
    }
}
