use amethyst::prelude::*;
use character_selection::CharacterSelectionState;
use game_play_state::GamePlayState;

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
    pub fn trans(&self) -> Trans<GameData<'static, 'static>> {
        match *self {
            Index::StartGame => {
                let next_state_fn = || Box::new(GamePlayState::new()); // kcov-ignore
                let character_selection_state =
                    Box::new(CharacterSelectionState::new(Box::new(next_state_fn)));
                Trans::Push(character_selection_state)
            }
            Index::Exit => Trans::Quit,
        }
    } // kcov-ignore
}

#[cfg(test)]
mod test {
    use amethyst::prelude::*;
    use debug_util_amethyst::assert_eq_trans;

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
    fn start_game_trans_returns_push() {
        assert_eq_trans(&Trans::Push(Box::new(MockState)), &Index::StartGame.trans());
    }

    #[test]
    fn exit_trans_returns_quit() {
        assert_eq_trans(&Trans::Quit, &Index::Exit.trans());
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>> for MockState {}
}
