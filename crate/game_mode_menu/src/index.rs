use amethyst::prelude::*;
use application_event::AppEvent;
use character_selection::{
    CharacterSelectionBundle, CharacterSelectionStateBuilder, CharacterSelectionStateDelegate,
};
use character_selection_ui::CharacterSelectionUiBundle;
use game_loading::GameLoadingState;
use game_play::GamePlayState;
use map_selection::{MapSelectionBundle, MapSelectionStateBuilder, MapSelectionStateDelegate};
use map_selection_ui::MapSelectionUiBundle;

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
    pub fn title(self) -> &'static str {
        match self {
            Index::StartGame => "Start Game",
            Index::Exit => "Exit",
        }
    } // kcov-ignore

    /// Returns the transition when this index has been selected.
    pub fn trans(self) -> Trans<GameData<'static, 'static>, AppEvent> {
        match self {
            Index::StartGame => {
                // kcov-ignore-start
                let game_play_fn = || Box::new(GamePlayState::new());
                let game_loading_fn = move || Box::new(GameLoadingState::new(game_play_fn));
                let map_selection_fn = move || {
                    let state = MapSelectionStateBuilder::new(MapSelectionStateDelegate::new(
                        game_loading_fn,
                    )).with_bundle(MapSelectionUiBundle::new())
                    .with_bundle(
                        MapSelectionBundle::new()
                            .with_system_dependencies(&MapSelectionUiBundle::system_names()),
                    ).build();

                    Box::new(state)
                };
                // kcov-ignore-end
                let character_selection_state = {
                    let state = CharacterSelectionStateBuilder::new(
                        CharacterSelectionStateDelegate::new(map_selection_fn),
                    ).with_bundle(CharacterSelectionUiBundle::new())
                    .with_bundle(
                        CharacterSelectionBundle::new()
                            .with_system_dependencies(&CharacterSelectionUiBundle::system_names()),
                    ).build();
                    Box::new(state)
                };

                Trans::Push(character_selection_state)
            }
            Index::Exit => Trans::Quit,
        }
    } // kcov-ignore
}

#[cfg(test)]
mod test {
    use amethyst::prelude::*;
    use application_event::AppEvent;
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
        assert_eq_trans(&Trans::Quit as &Trans<_, _>, &Index::Exit.trans());
    }

    #[derive(Debug)]
    struct MockState;
    impl<'a, 'b> State<GameData<'a, 'b>, AppEvent> for MockState {}
}
