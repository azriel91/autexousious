use amethyst::{GameData, Trans};
use application_event::AppEvent;
use character_selection::{
    CharacterSelectionBundle, CharacterSelectionStateBuilder, CharacterSelectionStateDelegate,
};
use character_selection_ui::CharacterSelectionUiBundle;
use game_loading::GameLoadingState;
use game_mode_selection_model::GameModeIndex;
use game_play::GamePlayState;
use map_selection::{MapSelectionBundle, MapSelectionStateBuilder, MapSelectionStateDelegate};
use map_selection_ui::MapSelectionUiBundle;

/// Returns the `Trans` for a given `GameModeIndex`.
#[derive(Debug)]
pub struct GameModeSelectionTrans;

impl GameModeSelectionTrans {
    /// Returns a transition when a menu item has been selected.
    ///
    /// # Parameters
    ///
    /// * `game_mode_index`: The selected index.
    pub fn trans(game_mode_index: GameModeIndex) -> Trans<GameData<'static, 'static>, AppEvent> {
        match game_mode_index {
            GameModeIndex::StartGame => {
                // kcov-ignore-start
                let game_play_fn = || Box::new(GamePlayState::new());
                let game_loading_fn = move || Box::new(GameLoadingState::new(game_play_fn));
                let map_selection_fn = move || {
                    let state = MapSelectionStateBuilder::new(MapSelectionStateDelegate::new(
                        game_loading_fn,
                    ))
                    .with_bundle(MapSelectionUiBundle::new())
                    .with_bundle(
                        MapSelectionBundle::new()
                            .with_system_dependencies(&MapSelectionUiBundle::system_names()),
                    )
                    .build();

                    Box::new(state)
                };
                // kcov-ignore-end
                let character_selection_state = {
                    let state = CharacterSelectionStateBuilder::new(
                        CharacterSelectionStateDelegate::new(map_selection_fn),
                    )
                    .with_bundle(CharacterSelectionUiBundle::new())
                    .with_bundle(
                        CharacterSelectionBundle::new()
                            .with_system_dependencies(&CharacterSelectionUiBundle::system_names()),
                    )
                    .build();

                    Box::new(state)
                };

                Trans::Push(character_selection_state)
            }
            GameModeIndex::Exit => Trans::Quit,
        }
    } // kcov-ignore
}
